use specs::prelude::*;
use specs_derive::Component;

use super::*;
use crate::atlas::Point;
use crate::clash::{EventCoordinator, Framer, Logger};

#[derive(Clone, Copy)]
pub enum BoltColor {
    Fire,
}

#[derive(Component)]
pub struct AttackComponent {
    pub strength: u32,
    pub color: BoltColor,
}

impl AttackComponent {
    pub fn init(strength: u32, color: BoltColor) -> AttackComponent {
        AttackComponent { strength, color }
    }
}

pub fn apply_bolt(ecs: &mut World, _bolt: &Entity, target: Point) {
    ecs.log(format!("Enemy was struck at ({},{})!", target.x, target.y).as_str());
}

pub fn begin_bolt(ecs: &mut World, source: &Entity, target: Point, strength: u32, color: BoltColor) {
    let initial = ecs.get_position(source);
    let path_length = initial.distance_to(target).unwrap() as u64;
    let frame = ecs.get_current_frame();
    let animation_length = if frame < 4 { 4 * path_length } else { 2 * path_length };

    let bolt = ecs
        .create_entity()
        .with(PositionComponent::init(initial))
        .with(AnimationComponent::bolt(initial.origin, target, frame, frame + animation_length))
        .with(AttackComponent::init(strength, color))
        .build();

    ecs.fire_event(EventType::Bolt, &bolt);
}
