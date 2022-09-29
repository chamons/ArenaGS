use std::{collections::HashSet, time::Duration};

use bevy_ecs::prelude::*;
use ggez::{glam::Vec2, mint::Vector2};
use keyframe::{functions::Linear, AnimationSequence, CanTween, Keyframe};
use keyframe_derive::CanTween;

use crate::core::{AnimationState, Appearance, Point, SizedPoint};

use super::{MovementAnimationComplete, MovementAnimationEvent, SpriteAnimateActionCompleteEvent, SpriteAnimateActionEvent};

#[derive(CanTween, Debug, Clone)]
pub struct MovementAnimation {
    pub animation: Vector2<f32>,
}

impl MovementAnimation {
    pub fn new(animation: Vector2<f32>) -> Self {
        Self { animation }
    }
}

impl From<Point> for MovementAnimation {
    fn from(point: Point) -> Self {
        Self {
            animation: Vector2 {
                x: point.x as f32,
                y: point.y as f32,
            },
        }
    }
}

impl Default for MovementAnimation {
    fn default() -> Self {
        Self {
            animation: Vector2 { x: 0.0, y: 0.0 },
        }
    }
}

// Need to use a wrapper struct as Vector2<f32> does not implement default

// Need a system to tweet start to end
// Need to give an offset for render_sprite

#[derive(Component)]
pub struct Animation {
    pub sprite: Option<AnimationSequence<f32>>,
    pub movement: Option<AnimationSequence<MovementAnimation>>,
}

impl Animation {
    pub fn new(sprite: Option<AnimationSequence<f32>>) -> Self {
        Animation { sprite, movement: None }
    }

    pub fn create_movement_animation(&mut self, start: Point, end: Point, duration: f32) {
        let frames: Vec<Keyframe<MovementAnimation>> = vec![(start.into(), 0.0, Linear).into(), (end.into(), duration, Linear).into()];
        self.movement = Some(AnimationSequence::from(frames));
    }
}

pub fn create_needed_idle_animations(world: &mut World) {
    let mut query = world.query::<(Entity, &Appearance, Option<&mut Animation>)>();
    let mut needs_sprite_animations = vec![];
    for (entity, appearance, mut animations) in query.iter_mut(world) {
        if let Some(animations) = animations.as_mut() {
            if animations.sprite.is_none() {
                animations.sprite = Some(appearance.create_standard_sprite_animation())
            }
        } else {
            needs_sprite_animations.push((entity, appearance.create_standard_sprite_animation()));
        }
    }
    for (entity, animation) in needs_sprite_animations {
        world.get_entity_mut(entity).unwrap().insert(Animation::new(Some(animation)));
    }
}

pub fn advance_all_animations(world: &mut World) {
    create_needed_idle_animations(world);

    let mut query = world.query::<(Entity, &Appearance, &mut Animation)>();
    let mut sprite_completed = vec![];
    let mut movement_completed = vec![];

    for (entity, appearance, mut animation) in query.iter_mut(world) {
        let should_loop = matches!(appearance.state, AnimationState::Idle);

        if let Some(sprite_animation) = &mut animation.sprite {
            if should_loop {
                sprite_animation.advance_and_maybe_reverse(1.0);
            } else {
                let animation_complete_amount = sprite_animation.advance_by(1.0);
                if animation_complete_amount > 0.0 {
                    sprite_completed.push(entity);
                }
            }
        }
        if let Some(movement_animation) = &mut animation.movement {
            let animation_complete_amount = movement_animation.advance_by(1.0);
            if animation_complete_amount > 0.0 {
                animation.movement = None;
                movement_completed.push(entity);
            }
        }
    }
    for complete in sprite_completed {
        world.send_event(SpriteAnimateActionCompleteEvent::new(complete));
    }

    for complete in movement_completed {
        world.send_event(MovementAnimationComplete::new(complete));
    }
}

#[no_mangle]
pub fn start_sprite_animations(mut requests: EventReader<SpriteAnimateActionEvent>, mut query: Query<(Entity, &mut Appearance, &mut Animation)>) {
    for request in requests.iter() {
        if let Ok((_, mut appearance, mut animation)) = query.get_mut(request.entity) {
            appearance.state = request.state;
            animation.sprite = None;
        }
    }
}

#[no_mangle]
pub fn end_sprite_animation(mut requests: EventReader<SpriteAnimateActionCompleteEvent>, mut query: Query<(&mut Appearance, &mut Animation)>) {
    // Because we can note animations as complete in render thread, we can often get multiple
    // notifications of the same Entity being complete. This is fine as long as we de-duplicate them
    let requests: HashSet<SpriteAnimateActionCompleteEvent> = HashSet::from_iter(requests.iter().cloned());

    // Unlike other animations, the idle "bob" needs to be sync across all units for it
    // to look good. So if we have any animation end requests, find the first idle (if any)
    // and use it
    let existing_idle_animation = if !requests.is_empty() {
        query
            .iter()
            .filter_map(|(appearance, animation)| {
                if appearance.state == AnimationState::Idle {
                    if let Some(animation) = &animation.sprite {
                        return Some(animation.clone());
                    }
                }
                None
            })
            .next()
    } else {
        None
    };

    for request in requests.iter() {
        if let Ok((mut appearance, mut animation)) = query.get_mut(request.entity) {
            appearance.state = AnimationState::Idle;
            if existing_idle_animation.is_some() {
                animation.sprite = existing_idle_animation.clone();
            } else {
                animation.sprite = Some(appearance.create_idle_animation());
            }
        }
    }
}

const MOVEMENT_ANIMATION_DURATION: f32 = 12.0;

#[no_mangle]
pub fn start_movement_animations(mut requests: EventReader<MovementAnimationEvent>, mut query: Query<&mut Animation>) {
    for request in requests.iter() {
        if let Ok(mut animation) = query.get_mut(request.entity) {
            animation.create_movement_animation(request.start.origin, request.end.origin, MOVEMENT_ANIMATION_DURATION);
        }
    }
}
