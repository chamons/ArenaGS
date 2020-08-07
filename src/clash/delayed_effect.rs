use specs::prelude::*;
use specs_derive::Component;

#[derive(Clone, Copy)]
pub enum DelayedEffectKind {
    Move,
    ApplyBolt,
    ApplyMelee,
}

pub struct DelayedEffect {
    kind: DelayedEffectKind,
    target_frame: u64,
}

impl DelayedEffect {
    pub fn init(kind: DelayedEffectKind, frame: u64, delta_frame: u64) -> DelayedEffect {
        DelayedEffect {
            kind,
            target_frame: frame + delta_frame,
        }
    }

    pub fn is_on_frame(&self, frame: u64) -> bool {
        self.target_frame <= frame
    }
}

#[derive(Component)]
pub struct DelayedEffectComponent {
    effect: DelayedEffect,
}

impl DelayedEffectComponent {
    pub fn init(effect: DelayedEffect) -> DelayedEffectComponent {
        DelayedEffectComponent { effect }
    }
}

pub fn tick_delayed_effects(ecs: &mut World, frame: u64) {
    let mut delayed_effects = vec![];
    {
        let entities = ecs.read_resource::<specs::world::EntitiesRes>();
        let delayed = ecs.read_storage::<DelayedEffectComponent>();
        for (entity, delay) in (&entities, &delayed).join() {
            if delay.effect.is_on_frame(frame) {
                delayed_effects.push((entity, delay.effect.kind));
            }
        }
    }

    use super::combat;
    use super::physics;
    for (entity, kind) in delayed_effects {
        match kind {
            DelayedEffectKind::ApplyBolt => {
                combat::apply_bolt(ecs, &entity);
                ecs.delete_entity(entity).unwrap();
            }
            DelayedEffectKind::ApplyMelee => {
                combat::apply_melee(ecs, &entity);
            }
            DelayedEffectKind::Move => {
                physics::complete_move(ecs, &entity);
            }
        }
        ecs.write_storage::<DelayedEffectComponent>().remove(entity);
    }
}

#[cfg(test)]
mod tests {}
