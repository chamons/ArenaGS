use specs::prelude::*;

use super::*;
use crate::atlas::EasyECS;

pub fn move_orb(ecs: &mut World, entity: &Entity) {
    remove_stale_fields(ecs, entity);

    let current_position = ecs.get_position(entity).origin;
    let orbs = ecs.read_storage::<OrbComponent>();
    let orb = orbs.grab(*entity);
    let path = &orb.path;
    let current_index = path.iter().position(|&x| x == current_position);
    let speed = orb.speed;
}

fn remove_stale_fields(ecs: &mut World, entity: &Entity) {
    let mut stale = vec![];
    {
        let entities = ecs.read_resource::<specs::world::EntitiesRes>();
        let positions = ecs.read_storage::<PositionComponent>();
        let fields = ecs.read_storage::<FieldComponent>();
        let orbs = ecs.read_storage::<OrbComponent>();
        let last_point = &orbs.grab(*entity).path.last();
        for (entity, _, orb, _) in (&entities, &positions, &orbs, &fields).join() {
            if orb.path.last() == *last_point {
                stale.push(entity);
            }
        }
    }

    for s in stale {
        ecs.delete_entity(s).unwrap();
    }
}
