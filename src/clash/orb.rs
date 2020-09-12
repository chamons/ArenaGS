use std::cmp;

use specs::prelude::*;

use super::*;
use crate::atlas::{EasyECS, EasyMutECS, SizedPoint};

pub fn create_orb(ecs: &mut World, invoker: &Entity) -> Entity {
    let orb_component = ecs.read_storage::<OrbComponent>().grab(*invoker).clone();
    let attack_component = ecs.read_storage::<AttackComponent>().grab(*invoker).clone();

    let path = &orb_component.path;
    let caster_position = ecs.get_position(invoker);
    let starting_index = path.iter().position(|x| !caster_position.contains_point(x)).unwrap();
    let orb_position = path[starting_index];

    let orb = ecs
        .create_entity()
        .with(PositionComponent::init(SizedPoint::from(orb_position)))
        .with(attack_component)
        .with(orb_component)
        .with(BehaviorComponent::init(BehaviorKind::Orb))
        .with(TimeComponent::init(0))
        .with(FieldComponent::init())
        .build();
    add_orb_movement_fields(ecs, &orb, starting_index);

    orb
}

#[allow(clippy::needless_range_loop)]
pub fn move_orb(ecs: &mut World, entity: &Entity) {
    let orb = ecs.read_storage::<OrbComponent>().grab(*entity).clone();
    let current_position = ecs.get_position(entity).origin;
    let current_index = orb.path.iter().position(|&x| x == current_position).unwrap();
    let next_index = cmp::min(current_index + orb.speed as usize, orb.path.len() - 1);
    let at_end = next_index == orb.path.len() - 1;

    for i in current_index..=next_index {
        if find_character_at_location(ecs, orb.path[i]).is_some() {
            apply_orb(ecs, *entity, orb.path[i]);
            return;
        }
    }

    if !at_end {
        wait(ecs, *entity);
        add_orb_movement_fields(ecs, entity, next_index);
        begin_move(ecs, entity, SizedPoint::from(orb.path[next_index]), PostMoveAction::None);
    } else {
        ecs.delete_entity(*entity).unwrap();
    }
}

fn add_orb_movement_fields(ecs: &mut World, entity: &Entity, current: usize) {
    let mut fields_to_add = vec![];
    let orbs = ecs.write_storage::<OrbComponent>();
    let orb = orbs.grab(*entity);
    for i in 0..1 + (2 * orb.speed as usize) {
        let (r, g, b) = {
            if i < orb.speed as usize {
                (255, 0, 0)
            } else {
                (230, 150, 0)
            }
        };

        if let Some(field) = orb.path.get(current + i) {
            fields_to_add.push((Some(*field), (r, g, b)));
        }
    }
    let mut fields = ecs.write_storage::<FieldComponent>();
    let field_component = fields.grab_mut(*entity);
    field_component.fields.clear();
    for (p, (r, g, b)) in fields_to_add {
        field_component.fields.push((p, (r, g, b, 140)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas::Point;

    fn assert_field_exists(ecs: &World, x: u32, y: u32) {
        let fields = ecs.read_storage::<FieldComponent>();
        assert!((&fields).join().any(|f| f.fields.iter().any(|(p, _)| {
            if let Some(p) = p {
                p.x == x && p.y == y
            } else {
                false
            }
        })));
    }

    fn assert_field_count(ecs: &World, expected: usize) {
        let fields = ecs.read_storage::<FieldComponent>();
        let count: usize = (&fields).join().map(|f| f.fields.len()).sum();
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
        assert_field_count(&ecs, 5);
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
        assert_field_count(&ecs, 10);
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
        assert_field_count(&ecs, 5);
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
            assert_field_count(&ecs, 5);
            new_turn_wait_characters(&mut ecs);
        }
        assert_field_count(&ecs, 5);
        new_turn_wait_characters(&mut ecs);
        assert_field_count(&ecs, 4);
        new_turn_wait_characters(&mut ecs);
        assert_field_count(&ecs, 0);
    }

    // This test sets two bolts on the same path, with overlapping
    // fields showing the path. If they only check endpoint, one will blap another
    #[test]
    fn orb_path_remove_correct_path_only() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_map().build();

        let player = find_at(&ecs, 2, 2);
        begin_orb(&mut ecs, &player, Point::init(2, 10), Damage::init(2), OrbKind::Feather, 2, 12);
        wait_for_animations(&mut ecs);
        dump_all_position(&ecs);

        // Bolt #1 at (2,3)
        assert_field_exists(&ecs, 2, 3);
        assert_field_exists(&ecs, 2, 4);
        assert_field_exists(&ecs, 2, 5);
        assert_field_exists(&ecs, 2, 6);
        assert_field_exists(&ecs, 2, 7);

        // Bolt #1 to (2,5)
        new_turn_wait_characters(&mut ecs);
        wait_for_animations(&mut ecs);
        dump_all_position(&ecs);

        // Bolt #2 at (2,3)
        let player = find_at(&ecs, 2, 2);
        begin_orb(&mut ecs, &player, Point::init(2, 10), Damage::init(2), OrbKind::Feather, 2, 12);
        wait_for_animations(&mut ecs);
        dump_all_position(&ecs);

        // Bolt #1
        assert_field_exists(&ecs, 2, 5);
        assert_field_exists(&ecs, 2, 6);
        assert_field_exists(&ecs, 2, 7);
        assert_field_exists(&ecs, 2, 8);
        assert_field_exists(&ecs, 2, 9);

        // Bolt #2
        assert_field_exists(&ecs, 2, 3);
        assert_field_exists(&ecs, 2, 4);
        assert_field_exists(&ecs, 2, 5);
        assert_field_exists(&ecs, 2, 6);
        assert_field_exists(&ecs, 2, 7);

        dump_all_position(&ecs);
        new_turn_wait_characters(&mut ecs);
        dump_all_position(&ecs);
        tick_next_action(&mut ecs);
        wait_for_animations(&mut ecs);
        dump_all_position(&ecs);
        // Bolt #1 to (2,7)
        // Bolt #2 at (2,5)

        // Bolt #1
        assert_field_exists(&ecs, 2, 7);
        assert_field_exists(&ecs, 2, 8);
        assert_field_exists(&ecs, 2, 9);
        assert_field_exists(&ecs, 2, 10);
        assert_field_exists(&ecs, 2, 11);

        // Bolt #2
        assert_field_exists(&ecs, 2, 5);
        assert_field_exists(&ecs, 2, 6);
        assert_field_exists(&ecs, 2, 7);
        assert_field_exists(&ecs, 2, 8);
        assert_field_exists(&ecs, 2, 9);
    }
}
