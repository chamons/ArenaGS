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
    ecs.shovel(*target, PositionComponent::init(position));
    ecs.write_storage::<SkipRenderComponent>().remove(*target);
    ecs.write_storage::<FlightComponent>().remove(*target);
}
