use std::collections::HashSet;

use bevy_ecs::prelude::*;
use keyframe::AnimationSequence;

use crate::core::{AnimationState, Appearance};

pub struct SpriteAnimateActionEvent {
    pub entity: Entity,
    pub state: AnimationState,
}

impl SpriteAnimateActionEvent {
    pub fn new(entity: Entity, state: AnimationState) -> Self {
        SpriteAnimateActionEvent { entity, state }
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub struct SpriteAnimateActionCompleteEvent {
    pub entity: Entity,
}

impl SpriteAnimateActionCompleteEvent {
    pub fn new(entity: Entity) -> Self {
        SpriteAnimateActionCompleteEvent { entity }
    }
}

#[derive(Component)]
pub struct Animation {
    pub sprite: Option<AnimationSequence<f32>>,
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
        world.get_entity_mut(entity).unwrap().insert(Animation { sprite: Some(animation) });
    }
}

pub fn advance_all_animations(world: &mut World) {
    create_needed_idle_animations(world);

    let mut query = world.query::<(Entity, &Appearance, &mut Animation)>();
    let mut completed = vec![];
    for (entity, appearance, mut animation) in query.iter_mut(world) {
        let should_loop = matches!(appearance.state, AnimationState::Idle);

        if let Some(animation) = &mut animation.sprite {
            if should_loop {
                animation.advance_and_maybe_reverse(1.0);
            } else {
                let animation_complete_amount = animation.advance_by(1.0);
                if animation_complete_amount > 0.0 {
                    completed.push(entity);
                }
            }
        }
    }
    for complete in completed {
        world.send_event(SpriteAnimateActionCompleteEvent::new(complete));
    }
}

#[no_mangle]
pub fn start_animation(mut requests: EventReader<SpriteAnimateActionEvent>, mut query: Query<(Entity, &mut Appearance, &mut Animation)>) {
    for request in requests.iter() {
        if let Ok((_, mut appearance, mut animation)) = query.get_mut(request.entity) {
            appearance.state = request.state;
            animation.sprite = None;
        }
    }
}

#[no_mangle]
pub fn end_animation(mut requests: EventReader<SpriteAnimateActionCompleteEvent>, mut query: Query<(Entity, &mut Appearance, &mut Animation)>) {
    // Because we can note animations as complete in render thread, we can often get multiple
    // notifications of the same Entity being complete. This is fine as long as we de-duplicate them
    let requests: HashSet<SpriteAnimateActionCompleteEvent> = HashSet::from_iter(requests.iter().cloned());

    // Unlike other animations, the idle "bob" needs to be sync across all units for it
    // to look good. So if we have any animation end requests, find the first idle (if any)
    // and use it
    let existing_idle_animation = if !requests.is_empty() {
        query
            .iter()
            .filter_map(|(_, appearance, animation)| {
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
        if let Ok((_, mut appearance, mut animation)) = query.get_mut(request.entity) {
            appearance.state = AnimationState::Idle;
            if existing_idle_animation.is_some() {
                animation.sprite = existing_idle_animation.clone();
            } else {
                animation.sprite = Some(appearance.create_idle_animation());
            }
        }
    }
}
