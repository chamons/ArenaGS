use std::collections::HashSet;

use bevy_ecs::prelude::*;
use ggez::mint::Vector2;
use keyframe::{functions::Linear, AnimationSequence};
use keyframe_derive::CanTween;

use crate::core::{AnimationState, Appearance};

use super::{MovementAnimationComplete, MovementAnimationEvent, SpriteAnimateActionCompleteEvent, SpriteAnimateActionEvent};

#[derive(CanTween, Debug, Clone)]
pub struct MovementAnimation {
    pub animation: Vector2<f32>,
}

impl From<Vector2<f32>> for MovementAnimation {
    fn from(animation: Vector2<f32>) -> Self {
        Self { animation }
    }
}

impl Default for MovementAnimation {
    fn default() -> Self {
        Self {
            animation: Vector2 { x: 0.0, y: 0.0 },
        }
    }
}

#[derive(Component)]
pub struct Animation {
    pub sprite: Option<AnimationSequence<f32>>,
    pub movement: Option<AnimationSequence<MovementAnimation>>,
}

pub enum PostMovementActionKind {
    Despawn,
}

#[derive(Component)]
pub struct PostMovementAction {
    kind: PostMovementActionKind,
}

impl PostMovementAction {
    pub fn new(kind: PostMovementActionKind) -> Self {
        PostMovementAction { kind }
    }
}

impl Animation {
    pub fn new() -> Self {
        Animation { sprite: None, movement: None }
    }
}

pub fn create_movement_animation(start: Vector2<f32>, end: Vector2<f32>, duration: f32) -> AnimationSequence<MovementAnimation> {
    AnimationSequence::from(vec![(start.into(), 0.0, Linear).into(), (end.into(), duration, Linear).into()])
}

fn find_idle_animation(world: &mut World, entity: Entity) -> AnimationSequence<f32> {
    // Unlike other animations, the idle "bob" needs to be sync across all units for it
    // to look good. So if we have any animation end requests, find the first idle (if any)
    // and use it
    let mut query = world.query::<(&Appearance, &Animation)>();
    let existing_idle_animation = query
        .iter(world)
        .filter_map(|(appearance, animation)| {
            if appearance.state == AnimationState::Idle {
                if let Some(animation) = &animation.sprite {
                    return Some(animation.clone());
                }
            }
            None
        })
        .next();
    if let Some(existing_idle_animation) = existing_idle_animation {
        existing_idle_animation
    } else {
        world.get::<Appearance>(entity).unwrap().create_standard_sprite_animation()
    }
}

pub fn advance_all_animations(world: &mut World) {
    create_needed_idle_animations(world);

    advance_sprite_animations(world);
    advance_movement_animations(world);
}

pub fn create_needed_idle_animations(world: &mut World) {
    let mut query = world.query_filtered::<(Entity, &Animation), With<Appearance>>();
    let mut needs_sprite_animations = vec![];
    for (entity, animations) in query.iter_mut(world) {
        if animations.sprite.is_none() {
            needs_sprite_animations.push(entity);
        }
    }
    for entity in needs_sprite_animations {
        let animation = find_idle_animation(world, entity);
        world.get_mut::<Animation>(entity).unwrap().sprite = Some(animation);
    }
}

fn advance_sprite_animations(world: &mut World) {
    let mut sprite_completed = vec![];
    let mut query = world.query::<(Entity, &Appearance, &mut Animation)>();
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
    }
    for complete in sprite_completed {
        world.send_event(SpriteAnimateActionCompleteEvent::new(complete));
    }
}

fn advance_movement_animations(world: &mut World) {
    let mut movement_completed = vec![];
    let mut query = world.query::<(Entity, &mut Animation)>();
    for (entity, mut animation) in query.iter_mut(world) {
        if let Some(movement_animation) = &mut animation.movement {
            let animation_complete_amount = movement_animation.advance_by(1.0);
            if animation_complete_amount > 0.0 {
                movement_completed.push(entity);
            }
        }
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

const MOVEMENT_ANIMATION_DURATION: f32 = 10.0;
const LARGE_MOVEMENT_ANIMATION_DURATION: f32 = 5.0;

fn distance(left: Vector2<f32>, right: Vector2<f32>) -> f32 {
    f32::sqrt((left.x - right.x).powi(2) + (left.y - right.y).powi(2))
}

#[no_mangle]
pub fn start_movement_animations(mut requests: EventReader<MovementAnimationEvent>, mut query: Query<&mut Animation>) {
    for request in requests.iter() {
        let mut animation = query.get_mut(request.entity).expect("Starting movement animation on item without animation");

        let distance = distance(request.start, request.end);
        let duration = match distance {
            x if (0.9..1.1).contains(&x) => MOVEMENT_ANIMATION_DURATION,
            _ => LARGE_MOVEMENT_ANIMATION_DURATION * distance as f32,
        };
        animation.movement = Some(create_movement_animation(request.start, request.end, duration));
    }
}

#[no_mangle]
pub fn end_movement_animation(
    mut requests: EventReader<MovementAnimationComplete>,
    mut query: Query<(&mut Animation, Option<&mut PostMovementAction>)>,
    mut commands: Commands,
) {
    for request in requests.iter() {
        let entity = request.entity;
        if let Ok((mut animation, action)) = query.get_mut(entity) {
            animation.movement = None;
            if let Some(action) = action {
                match action.kind {
                    PostMovementActionKind::Despawn => commands.add(move |w: &mut World| {
                        w.despawn(entity);
                    }),
                }
            }
        }
    }
}
