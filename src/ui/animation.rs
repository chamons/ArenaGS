use bevy_ecs::prelude::*;

use crate::core::{appearance, AnimationState, Appearance};

pub struct SpriteAnimateActionEvent {
    pub entity: Entity,
    pub state: AnimationState,
}

impl SpriteAnimateActionEvent {
    pub fn new(entity: Entity, state: AnimationState) -> Self {
        SpriteAnimateActionEvent { entity, state }
    }
}

pub struct SpriteAnimateActionComplete {
    pub entity: Entity,
}

impl SpriteAnimateActionComplete {
    pub fn new(entity: Entity) -> Self {
        SpriteAnimateActionComplete { entity }
    }
}

pub fn advance_all_animations(world: &mut World) {
    let mut query = world.query::<(Entity, &mut Appearance)>();
    let mut completed = vec![];
    for (entity, mut appearance) in query.iter_mut(world) {
        if appearance.animation.is_none() {
            appearance.animation = Some(appearance.create_animation(None))
        }

        let loops = match appearance.state {
            AnimationState::Idle => true,
            _ => false,
        };
        if let Some(animation) = &mut appearance.animation {
            if loops {
                animation.advance_and_maybe_reverse(1.0);
            } else {
                if animation.advance_by(1.0) > 0.0 {
                    completed.push(entity);
                }
            }
        }
    }
    for complete in completed {
        world.send_event(SpriteAnimateActionComplete::new(complete));
    }
}

pub fn start_animation(mut requests: EventReader<SpriteAnimateActionEvent>, mut query: Query<(Entity, &mut Appearance)>) {
    for request in requests.iter() {
        if let Ok((_, mut appearance)) = query.get_mut(request.entity) {
            appearance.state = request.state;
            appearance.animation = None;
            println!("Setting animation: {:?}", request.state);
        }
    }
}

pub fn end_animation(mut requests: EventReader<SpriteAnimateActionComplete>, mut query: Query<(Entity, &mut Appearance)>) {
    // Unlike other animations, the idle "bob" needs to be sync across all units for it
    // to look good. So if we have any animation end requests, find the first idle (if any)
    // and use it's duration. Else default to zero.
    let idle_frame = if !requests.is_empty() {
        query
            .iter()
            .filter_map(|(_, a)| {
                if a.state == AnimationState::Idle {
                    if let Some(animation) = &a.animation {
                        return Some(animation.time());
                    }
                }
                None
            })
            .next()
    } else {
        None
    };

    for request in requests.iter() {
        if let Ok((_, mut appearance)) = query.get_mut(request.entity) {
            println!("Clear animation: {:?}", appearance.state);
            appearance.state = AnimationState::Idle;
            appearance.animation = Some(appearance.create_animation(idle_frame));
        }
    }
}
