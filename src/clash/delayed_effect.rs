use specs::prelude::*;
use specs_derive::Component;

#[derive(Clone, Copy)]
pub enum DelayedEffectKind {
    Move,
    ApplyBolt,
    ApplyMelee,
    #[cfg(test)]
    None,
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
            #[cfg(test)]
            DelayedEffectKind::None => {}
        }
        ecs.write_storage::<DelayedEffectComponent>().remove(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::super::create_test_state;
    use super::*;

    #[test]
    fn removed_on_tick() {
        let mut ecs = create_test_state().build();
        ecs.create_entity()
            .with(DelayedEffectComponent::init(DelayedEffect::init(DelayedEffectKind::None, 0, 10)))
            .build();
        for i in 0..11 {
            tick_delayed_effects(&mut ecs, i);
        }
        assert_eq!(0, ecs.read_storage::<DelayedEffectComponent>().count());
    }

    #[test]
    fn removed_if_past_tick() {
        let mut ecs = create_test_state().build();
        ecs.create_entity()
            .with(DelayedEffectComponent::init(DelayedEffect::init(DelayedEffectKind::None, 0, 10)))
            .build();
        for i in 0..10 {
            tick_delayed_effects(&mut ecs, i);
        }
        tick_delayed_effects(&mut ecs, 12);
        assert_eq!(0, ecs.read_storage::<DelayedEffectComponent>().count());
    }
}
