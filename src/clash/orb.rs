use std::cmp;

use specs::prelude::*;

use super::*;
use crate::atlas::{EasyECS, Point, SizedPoint};

#[allow(clippy::needless_range_loop)]
pub fn move_orb(ecs: &mut World, orb: &Entity) {
    remove_stale_fields(ecs, orb);

    let (path, speed, current_index, next_index, at_end) = {
        let current_position = ecs.get_position(orb).origin;
        let orbs = ecs.read_storage::<OrbComponent>();
        let orb = orbs.grab(*orb);
        let path = &orb.path;
        let current_index = path.iter().position(|&x| x == current_position).unwrap();
        let speed = orb.speed;
        let next_index = cmp::min(current_index + speed as usize, path.len() - 1);
        let at_end = next_index == path.len() - 1;
        (path.clone(), speed, current_index, next_index, at_end)
    };

    for i in current_index..=next_index {
        if find_character_at_location(ecs, path[i]).is_some() {
            apply_orb(ecs, *orb, path[i]);
            return;
        }
    }

    if !at_end {
        wait(ecs, *orb);
        add_orb_movement_fields(ecs, &path, speed, next_index);
        begin_move(ecs, orb, SizedPoint::from(path[next_index]), PostMoveAction::None);
    } else {
        ecs.delete_entity(*orb).unwrap();
    }
}

pub fn create_orb(ecs: &mut World, invoker: &Entity, attack: AttackInfo, path: &[Point]) -> Entity {
    let caster_position = ecs.get_position(invoker);
    let starting_index = path.iter().position(|x| !caster_position.contains_point(x)).unwrap();

    let speed = attack.orb_speed();
    let orb = ecs
        .create_entity()
        .with(PositionComponent::init(SizedPoint::from(path[starting_index])))
        .with(AttackComponent { attack })
        .with(OrbComponent::init(path.to_vec(), speed))
        .with(BehaviorComponent::init(BehaviorKind::Orb))
        .with(TimeComponent::init(0))
        .build();

    add_orb_movement_fields(ecs, &path, speed, starting_index);
    orb
}

fn add_orb_movement_fields(ecs: &mut World, path: &[Point], speed: u32, current: usize) {
    for i in 0..2 * speed as usize {
        let (r, g, b) = {
            if i < speed as usize {
                (255, 0, 0)
            } else {
                (128, 128, 0)
            }
        };

        if let Some(field) = path.get(current + i + 1) {
            ecs.create_entity()
                .with(PositionComponent::init(SizedPoint::from(*field)))
                .with(FieldComponent::init(r, g, b))
                .with(OrbComponent::init(path.to_vec(), speed))
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

    fn assert_field_exists(ecs: &World, x: u32, y: u32) {
        let fields = ecs.read_storage::<FieldComponent>();
        let positions = ecs.read_storage::<PositionComponent>();
        assert!((&fields, &positions).join().any(|(_, p)| p.position.origin.x == x && p.position.origin.y == y));
    }

    fn assert_field_count(ecs: &World, expected: usize) {
        let fields = ecs.read_storage::<FieldComponent>();
        let count = (&fields).join().count();
        assert_eq!(expected, count);
    }

    #[test]
    fn orb_has_correct_fields() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 6, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 2);
        wait_for_animations(&mut ecs);
        assert_field_exists(&ecs, 2, 4);
        assert_field_exists(&ecs, 2, 5);
        assert_field_exists(&ecs, 2, 6);
        assert_field_count(&ecs, 3);
    }

    #[test]
    fn super_fast_orb_has_correct_fields() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 6, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 8);
        wait_for_animations(&mut ecs);
        assert_field_exists(&ecs, 2, 4);
        assert_field_exists(&ecs, 2, 5);
        assert_field_exists(&ecs, 2, 6);
        assert_field_count(&ecs, 3);
    }

    #[test]
    fn orb_from_multi_sized_has_correct_fields() {
        let mut ecs = create_test_state()
            .with_sized_character(SizedPoint::init_multi(2, 6, 2, 2), 0)
            .with_character(2, 2, 0)
            .with_map()
            .build();
        let invoker = find_at(&ecs, 2, 6);

        begin_orb(&mut ecs, &invoker, Point::init(2, 2), Damage::init(2), OrbKind::Feather, 2);
        wait_for_animations(&mut ecs);
        assert_field_exists(&ecs, 2, 3);
        assert_field_exists(&ecs, 2, 2);
        assert_field_count(&ecs, 2);
    }

    #[test]
    fn orb_removes_fields_on_hit() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 6, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 8);
        wait_for_animations(&mut ecs);
        new_turn_wait_characters(&mut ecs);
        assert_field_count(&ecs, 0);
    }

    #[test]
    fn orb_removes_fields_on_move() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 10, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        begin_orb(&mut ecs, &player, Point::init(2, 10), Damage::init(2), OrbKind::Feather, 2);
        wait_for_animations(&mut ecs);

        for _ in 0..2 {
            assert_field_count(&ecs, 4);
            new_turn_wait_characters(&mut ecs);
        }
        assert_field_count(&ecs, 3);
        new_turn_wait_characters(&mut ecs);
        assert_field_count(&ecs, 1);
        new_turn_wait_characters(&mut ecs);
        assert_field_count(&ecs, 0);
    }
}
