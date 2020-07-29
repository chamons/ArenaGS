use specs::prelude::*;
use specs_derive::Component;

use super::*;
use crate::atlas::Point;
use crate::clash::{find_character_at_location, EventCoordinator, Framer, Logger};

#[derive(Clone, Copy)]
pub enum BoltKind {
    Fire,
}

#[derive(Clone, Copy)]
pub enum WeaponKind {
    Sword,
}

#[derive(Component)]
pub struct AttackComponent {
    pub strength: u32,
    pub color: BoltKind,
}

impl AttackComponent {
    pub fn init(strength: u32, color: BoltKind) -> AttackComponent {
        AttackComponent { strength, color }
    }
}

pub fn apply_bolt(ecs: &mut World, bolt: &Entity, target: Point) {
    let attack = {
        let attacks = ecs.read_storage::<AttackComponent>();
        attacks.get(*bolt).unwrap().strength
    };

    ecs.log(format!("Enemy was struck ({}) at ({},{})!", attack, target.x, target.y).as_str());
}

pub fn begin_bolt(ecs: &mut World, source: &Entity, target: Point, strength: u32, kind: BoltKind) {
    let initial = ecs.get_position(source);
    let path_length = initial.distance_to(target).unwrap() as u64;
    let frame = ecs.get_current_frame();
    let animation_length = if frame < 4 { 4 * path_length } else { 2 * path_length };

    let bolt = ecs
        .create_entity()
        .with(PositionComponent::init(initial))
        .with(AnimationComponent::bolt(initial.origin, target, frame, animation_length))
        .with(AttackComponent::init(strength, kind))
        .build();

    ecs.fire_event(EventType::Bolt(*source), &bolt);
}

pub fn begin_melee(ecs: &mut World, source: &Entity, target: Point, strength: u32, kind: WeaponKind) {
    if let Some(target_characater) = find_character_at_location(ecs, target) {
        ecs.fire_event(EventType::Melee(*source, strength, kind), &target_characater);
    }
}

pub fn apply_melee(ecs: &mut World, _target_character: &Entity, target_point: Point, strength: u32, _kind: WeaponKind) {
    ecs.log(format!("Enemy was struck ({}) in melee at ({},{})!", strength, target_point.x, target_point.y).as_str());
}
