use std::cmp;

use specs::prelude::*;

use super::*;
use crate::atlas::{EasyECS, Point, SizedPoint};

pub fn create_orb(ecs: &mut World, invoker: &Entity) -> Entity {
    let orb_component = ecs.read_storage::<OrbComponent>().grab(*invoker).clone();
    let attack_component = ecs.read_storage::<AttackComponent>().grab(*invoker).clone();

    let path = &orb_component.path;
    let caster_position = ecs.get_position(invoker);
    let starting_index = path.iter().position(|x| !caster_position.contains_point(x)).unwrap();

    add_orb_movement_fields(ecs, &orb_component, starting_index);
    let orb = ecs
        .create_entity()
        .with(PositionComponent::init(SizedPoint::from(path[starting_index])))
        .with(attack_component)
        .with(orb_component)
        .with(BehaviorComponent::init(BehaviorKind::Orb))
        .with(TimeComponent::init(0))
        .build();

    orb
}

#[allow(clippy::needless_range_loop)]
pub fn move_orb(ecs: &mut World, entity: &Entity) {
    remove_stale_fields(ecs, entity);

    let orb = ecs.read_storage::<OrbComponent>().grab(*entity).clone();
    let (current_index, next_index, at_end) = {
        let current_position = ecs.get_position(entity).origin;
        let current_index = orb.path.iter().position(|&x| x == current_position).unwrap();
        let next_index = cmp::min(current_index + orb.speed as usize, orb.path.len() - 1);
        let at_end = next_index == orb.path.len() - 1;
        (current_index, next_index, at_end)
    };

    for i in current_index..=next_index {
        if find_character_at_location(ecs, orb.path[i]).is_some() {
            apply_orb(ecs, *entity, orb.path[i]);
            return;
        }
    }

    if !at_end {
        wait(ecs, *entity);
        add_orb_movement_fields(ecs, &orb, next_index);
        begin_move(ecs, entity, SizedPoint::from(orb.path[next_index]), PostMoveAction::None);
    } else {
        ecs.delete_entity(*entity).unwrap();
    }
}

fn add_orb_movement_fields(ecs: &mut World, orb: &OrbComponent, current: usize) {
    for i in 0..2 * orb.speed as usize {
        let (r, g, b) = {
            if i < orb.speed as usize {
                (255, 0, 0)
            } else {
                (230, 150, 0)
            }
        };

        if let Some(field) = orb.path.get(current + i + 1) {
            ecs.create_entity()
                .with(PositionComponent::init(SizedPoint::from(*field)))
                .with(FieldComponent::init(r, g, b))
                .with(orb.clone())
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

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 2, 12);
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

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 8, 12);
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

        begin_orb(&mut ecs, &invoker, Point::init(2, 2), Damage::init(2), OrbKind::Feather, 2, 12);
        wait_for_animations(&mut ecs);
        assert_field_exists(&ecs, 2, 3);
        assert_field_exists(&ecs, 2, 2);
        assert_field_count(&ecs, 2);
    }

    #[test]
    fn orb_removes_fields_on_hit() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 6, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        begin_orb(&mut ecs, &player, Point::init(2, 6), Damage::init(2), OrbKind::Feather, 8, 12);
        wait_for_animations(&mut ecs);
        new_turn_wait_characters(&mut ecs);
        assert_field_count(&ecs, 0);
    }

    #[test]
    fn orb_removes_fields_on_move() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 10, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);

        begin_orb(&mut ecs, &player, Point::init(2, 10), Damage::init(2), OrbKind::Feather, 2, 12);
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
