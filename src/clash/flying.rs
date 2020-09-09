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

fn end_flight(ecs: &mut World, target: &Entity) {
    let position = ecs.read_storage::<FlightComponent>().grab(*target).takeoff_point;
    let position = find_clear_landing(ecs, &position, Some(*target));
    ecs.shovel(*target, PositionComponent::init(position));
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
