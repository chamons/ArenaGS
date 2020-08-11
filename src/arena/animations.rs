use specs::prelude::*;
use specs_derive::Component;

use crate::after_image::CharacterAnimationState;
use crate::atlas::Point;
use crate::clash::{EventCoordinator, EventKind};

#[derive(PartialEq)]
pub struct FPoint {
    pub x: f32,
    pub y: f32,
}

impl FPoint {
    pub const fn init(x: f32, y: f32) -> FPoint {
        FPoint { x, y }
    }
}

#[derive(Clone, Copy)]
pub enum AnimationKind {
    None, // Example - A explosion that changes sprite but no movement
    Position {
        start: Point,
        end: Point,
    },
    CharacterState {
        now: CharacterAnimationState,
        done: CharacterAnimationState,
    },
}

#[derive(Clone, Copy)]
pub struct PostAnimationEvent {
    kind: EventKind,
    target: Option<Entity>,
}

pub struct Animation {
    pub kind: AnimationKind,
    pub beginning: u64,
    pub duration: u64,
    pub post_event: Option<PostAnimationEvent>,
}

impl Animation {
    pub fn is_complete(&self, current: u64) -> bool {
        (current - self.beginning) > self.duration
    }

    pub fn current_position(&self, current: u64) -> Option<FPoint> {
        match &self.kind {
            AnimationKind::Position { start, end } => {
                if self.is_complete(current) {
                    Some(FPoint::init(end.x as f32, end.y as f32))
                } else {
                    let delta = (current - self.beginning) as f64 / self.duration as f64;
                    let x = lerp(start.x as f32, end.x as f32, delta);
                    let y = lerp(start.y as f32, end.y as f32, delta);
                    Some(FPoint::init(x, y))
                }
            }
            _ => None,
        }
    }

    pub fn current_character_state(&self) -> Option<&CharacterAnimationState> {
        match &self.kind {
            AnimationKind::CharacterState { now, done: _ } => Some(now),
            // Bit of a hack since we can't have multiple animations stacked
            AnimationKind::Position { .. } => Some(&CharacterAnimationState::Walk),
            AnimationKind::None => None,
        }
    }
}

fn lerp(start: f32, end: f32, t: f64) -> f32 {
    (start as f64 * (1.0f64 - t) + end as f64 * t) as f32
}

#[derive(Component)]
pub struct AnimationComponent {
    pub animation: Animation,
}

impl AnimationComponent {
    pub fn movement(start_point: Point, end_point: Point, beginning: u64, duration: u64) -> AnimationComponent {
        AnimationComponent {
            animation: Animation {
                kind: AnimationKind::Position {
                    start: start_point,
                    end: end_point,
                },
                beginning,
                duration,
                post_event: None,
            },
        }
    }

    pub fn sprite_state(now: CharacterAnimationState, done: CharacterAnimationState, beginning: u64, duration: u64) -> AnimationComponent {
        AnimationComponent {
            animation: Animation {
                kind: AnimationKind::CharacterState { now, done },
                beginning,
                duration,
                post_event: None,
            },
        }
    }

    pub fn empty(beginning: u64, duration: u64) -> AnimationComponent {
        AnimationComponent {
            animation: Animation {
                kind: AnimationKind::None,
                beginning,
                duration,
                post_event: None,
            },
        }
    }

    pub fn with_post_event(mut self, kind: EventKind, target: Option<Entity>) -> AnimationComponent {
        self.animation.post_event = Some(PostAnimationEvent { kind, target });
        self
    }
}

pub fn tick_animations(ecs: &mut World, frame: u64) {
    let mut completed = vec![];
    let mut needs_events = vec![];
    {
        let entities = ecs.read_resource::<specs::world::EntitiesRes>();
        let animations = ecs.read_storage::<AnimationComponent>();

        for (entity, animation_component) in (&entities, &animations).join() {
            let animation = &animation_component.animation;
            if animation.is_complete(frame) {
                completed.push(entity);
                if let Some(post_event) = animation.post_event {
                    needs_events.push(post_event);
                }
            }
        }
    }

    for need_events in needs_events {
        ecs.raise_event(need_events.kind, need_events.target);
    }

    for entity in &completed {
        ecs.write_storage::<AnimationComponent>().remove(*entity);
    }
}

pub fn complete_animations(ecs: &mut World) {
    loop {
        let current_frame = {
            ecs.write_resource::<crate::clash::FrameComponent>().current_frame += 1;
            ecs.read_resource::<crate::clash::FrameComponent>().current_frame
        };

        super::battle_scene::process_tick_events(ecs, current_frame);

        let animations = ecs.read_storage::<AnimationComponent>();
        if (animations).join().count() == 0 {
            break;
        }
    }
}
