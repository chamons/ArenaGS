use specs::prelude::*;
use specs_derive::Component;

use super::{complete_move, PositionComponent};
use crate::after_image::CharacterAnimationState;
use crate::atlas::BoxResult;
use crate::clash::Point;

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

pub enum Animation {
    Position {
        start: Point,
        end: Point,
    },
    CharacterState {
        now: CharacterAnimationState,
        done: CharacterAnimationState,
    },
}

#[derive(Component)]
pub struct AnimationComponent {
    pub animation: Animation,
    pub beginning: u64,
    pub ending: u64,
}

impl AnimationComponent {
    pub fn movement(start_point: Point, end_point: Point, beginning: u64, ending: u64) -> AnimationComponent {
        AnimationComponent {
            animation: Animation::Position {
                start: start_point,
                end: end_point,
            },
            beginning,
            ending,
        }
    }

    #[allow(dead_code)]
    pub fn sprite_state(now: CharacterAnimationState, done: CharacterAnimationState, beginning: u64, ending: u64) -> AnimationComponent {
        AnimationComponent {
            animation: Animation::CharacterState { now, done },
            beginning,
            ending,
        }
    }

    pub fn is_complete(&self, current: u64) -> bool {
        current > self.ending
    }

    pub fn current_position(&self, current: u64) -> Option<FPoint> {
        match &self.animation {
            Animation::Position { start, end } => {
                if self.is_complete(current) {
                    Some(FPoint::init(end.x as f32, end.y as f32))
                } else {
                    let delta = (current - self.beginning) as f64 / (self.ending - self.beginning) as f64;
                    let x = lerp(start.x as f32, end.x as f32, delta);
                    let y = lerp(start.y as f32, end.y as f32, delta);
                    Some(FPoint::init(x, y))
                }
            }
            _ => None,
        }
    }

    pub fn current_character_state(&self) -> Option<&CharacterAnimationState> {
        match &self.animation {
            Animation::CharacterState { now, done: _ } => Some(now),
            // Bit of a hack since we can't have multiple animations stacked
            Animation::Position { start: _, end: _ } => Some(&CharacterAnimationState::Walk),
        }
    }
}

fn lerp(start: f32, end: f32, t: f64) -> f32 {
    (start as f64 * (1.0f64 - t) + end as f64 * t) as f32
}

pub fn tick_animations(ecs: &World, frame: u64) -> BoxResult<()> {
    // Remove completed animations, applying their change
    let mut completed = vec![];
    let mut to_move = vec![];
    {
        let entities = ecs.read_resource::<specs::world::EntitiesRes>();
        let animations = ecs.read_storage::<AnimationComponent>();
        let mut positions = ecs.write_storage::<PositionComponent>();

        for (entity, animation, position) in (&entities, &animations, (&mut positions).maybe()).join() {
            if animation.is_complete(frame) {
                completed.push(entity);
            }
            if let Animation::Position { start: _, end } = &animation.animation {
                if position.is_some() {
                    to_move.push((entity, *end));
                }
            }
        }
    }
    for (entity, position) in to_move.iter() {
        complete_move(ecs, entity, position);
    }
    {
        let mut animations = ecs.write_storage::<AnimationComponent>();
        for c in completed {
            animations.remove(c);
        }
    }

    Ok(())
}
