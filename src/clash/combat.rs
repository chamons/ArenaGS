use specs::prelude::*;
use specs_derive::Component;

use super::*;
use crate::atlas::Point;
use crate::clash::Logger;

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
    const BOLT_SPEED_PER_TILE: u32 = 10;

    let sprite = match color {
        BoltColor::Fire => SpriteKinds::FireBolt,
    };
    let initial = ecs.get_position(source);

    // Hard problem - We need to attach a render component with a sprite
    // However, we're in clash / guts of combat, we don't have access
    // How we we hook in something to let us communicate up - callback? event?
    // This will be the first of many
    ecs.create_entity()
        .with(RenderComponent::init(sprite))
        .with(PositionComponent::init(initial))
        .with(AnimationComponent::bolt(initial.origin, target, 0, 40))
        .build();
}
