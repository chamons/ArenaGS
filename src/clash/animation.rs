use specs::prelude::*;
use specs_derive::Component;

use super::{EventCoordinator, EventKind};
use crate::after_image::CharacterAnimationState;
use crate::atlas::BoxResult;
use crate::atlas::Point;

// Animations are a strange beast/
// Unless we do some sort of late binding,
// if we want an action, say a move, to animate
// to the new state before applying the new location
// the engine needs to create them
// However, they really are UI constructs

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
pub enum PostAnimationEffect {
    None,
    Move,
    StartBolt,
    ApplyBolt,
    ApplyMelee,
}

pub struct Animation {
    pub kind: AnimationKind,
    pub beginning: u64,
    pub duration: u64,
    pub effect: PostAnimationEffect,
}

impl Animation {
    pub fn end_position(&self) -> Point {
        match self.kind {
            AnimationKind::Position { end, .. } => end,
            _ => panic!("Wrong kind in end_position"),
        }
    }

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
        }
    }
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
                effect: PostAnimationEffect::None,
            },
        }
    }
    pub fn sprite_state(now: CharacterAnimationState, done: CharacterAnimationState, beginning: u64, duration: u64) -> AnimationComponent {
        AnimationComponent {
            animation: Animation {
                kind: AnimationKind::CharacterState { now, done },
                beginning,
                duration,
                effect: PostAnimationEffect::None,
            },
        }
    }

    pub fn with_effect(mut self, effect: PostAnimationEffect) -> AnimationComponent {
        self.animation.effect = effect;
        self
    }
}

fn lerp(start: f32, end: f32, t: f64) -> f32 {
    (start as f64 * (1.0f64 - t) + end as f64 * t) as f32
}

pub fn tick_animations(ecs: &mut World, frame: u64) -> BoxResult<()> {
    let mut completed = vec![];
    {
        let entities = ecs.read_resource::<specs::world::EntitiesRes>();
        let animations = ecs.read_storage::<AnimationComponent>();

        for (entity, animation_component) in (&entities, &animations).join() {
            let animation = &animation_component.animation;
            if animation.is_complete(frame) {
                completed.push((entity, animation.effect));
            }
        }
    }

    for (entity, effect) in completed {
        ecs.fire_event(EventKind::AnimationComplete(entity, effect), &entity);
        ecs.write_storage::<AnimationComponent>().remove(entity);
    }

    Ok(())
}
