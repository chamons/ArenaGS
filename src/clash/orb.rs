use std::cmp;

use specs::prelude::*;

use super::*;
use crate::atlas::{EasyECS, Point, SizedPoint};

pub fn move_orb(ecs: &mut World, entity: &Entity) {
    remove_stale_fields(ecs, entity);

    let (path, speed, current_index, next_index) = {
        let current_position = ecs.get_position(entity).origin;
        let orbs = ecs.read_storage::<OrbComponent>();
        let orb = orbs.grab(*entity);
        let path = &orb.path;
        let current_index = path.iter().position(|&x| x == current_position).unwrap();
        let speed = orb.speed;
        let next_index = cmp::min(current_index + speed as usize, path.len() - 1);
        (path.clone(), speed, current_index, next_index)
    };

    for i in current_index..=next_index {
        if find_character_at_location(ecs, path[i]).is_some() {
            apply_orb(ecs, *entity, path[i]);
            return;
        }
    }

    wait(ecs, *entity);
    add_orb_movement_fields(ecs, &path, speed, next_index);
    begin_move(ecs, entity, SizedPoint::from(path[next_index]), PostMoveAction::None);
}

pub fn create_orb(ecs: &mut World, attack: AttackInfo, path: &Vec<Point>, position: &Point) -> Entity {
    let speed = attack.orb_speed();
    let orb = ecs
        .create_entity()
        .with(PositionComponent::init(SizedPoint::from(*position)))
        .with(AttackComponent { attack })
        .with(OrbComponent::init(path.clone(), speed))
        .with(BehaviorComponent::init(BehaviorKind::Orb))
        .with(TimeComponent::init(-100))
        .build();

    add_orb_movement_fields(ecs, &path, speed, 1);
    orb
}

fn add_orb_movement_fields(ecs: &mut World, path: &Vec<Point>, speed: u32, current: usize) {
    for i in 0..speed as usize {
        if let Some(field) = path.get(current + i + 1) {
            ecs.create_entity()
                .with(PositionComponent::init(SizedPoint::from(*field)))
                .with(FieldComponent::init(255, 0, 0))
                .with(OrbComponent::init(path.clone(), speed))
                .build();
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orb_has_correct_fields() {}

    #[test]
    fn orb_removes_fields_on_hit() {}

    #[test]
    fn orb_removes_fields_on_move() {}
}
