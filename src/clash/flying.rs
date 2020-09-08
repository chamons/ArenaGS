use specs::prelude::*;

use super::*;
use crate::atlas::{EasyECS, EasyMutWorld, SizedPoint};

pub fn flying_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::StatusAdded(kind) => {
            if kind.is_flying() {
                start_flight(ecs, &target.unwrap())
            }
        }
        EventKind::StatusRemoved(kind) => {
            if kind.is_flying() {
                end_flight(ecs, &target.unwrap())
            }
        }
        EventKind::StatusExpired(kind) => {
            if kind.is_flying() {
                end_flight(ecs, &target.unwrap())
            }
        }
        _ => {}
    }
}

fn start_flight(ecs: &mut World, target: &Entity) {
    let position = ecs.get_position(target);
    ecs.shovel(*target, FlightComponent::init(position));
    ecs.shovel(*target, SkipRenderComponent::init());
    ecs.write_storage::<PositionComponent>().remove(*target);
}

fn find_clear_landing(ecs: &mut World, target: &Entity, initial: &SizedPoint) -> SizedPoint {
    for distance in 1..3 {
        for direction in get_random_direction_list(ecs) {
            let mut attempt = *initial;
            for _ in 0..distance {
                if let Some(p) = direction.sized_point_in_direction(&attempt) {
                    attempt = p;
                }
            }
            if is_area_clear(ecs, &attempt.all_positions(), target) {
                return attempt;
            }
        }
    }
    // This seems very unlikely, we check every single possibility within 3 of the takeoff point.
    panic!(
        "Unable to find clear landing for flying {} at {}",
        ecs.get_name(target).unwrap(),
        initial.origin
    );
}

fn end_flight(ecs: &mut World, target: &Entity) {
    let position = ecs.read_storage::<FlightComponent>().grab(*target).takeoff_point;
    if is_area_clear(ecs, &position.all_positions(), target) {
        ecs.shovel(*target, PositionComponent::init(position));
    } else {
        let new_position = find_clear_landing(ecs, target, &position);
        ecs.shovel(*target, PositionComponent::init(new_position));
    }
    ecs.write_storage::<SkipRenderComponent>().remove(*target);
    ecs.write_storage::<FlightComponent>().remove(*target);
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::atlas::Point;

    #[test]
    fn flight_round_trip() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let enemy = find_at(&ecs, 2, 2);
        ecs.add_status(&enemy, StatusKind::Flying, 200);
        assert!(ecs.read_storage::<PositionComponent>().get(enemy).is_none());
        add_ticks(&mut ecs, 100);
        assert!(ecs.read_storage::<PositionComponent>().get(enemy).is_none());
        add_ticks(&mut ecs, 100);
        assert!(ecs.read_storage::<PositionComponent>().get(enemy).is_some());
    }

    #[test]
    fn flight_landing_taken() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_character(2, 3, 100).with_map().build();
        let enemy = find_at(&ecs, 2, 2);
        let bystander = find_at(&ecs, 2, 3);
        ecs.add_status(&enemy, StatusKind::Flying, 100);
        begin_move(&mut ecs, &bystander, SizedPoint::init(2, 2), PostMoveAction::None);
        wait_for_animations(&mut ecs);
        add_ticks(&mut ecs, 100);
        assert_not_at_position(&ecs, &enemy, Point::init(2, 2));
    }
}
