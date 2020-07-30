use specs::prelude::*;
use specs_derive::Component;

use super::*;
use crate::atlas::Point;
use crate::clash::{EventCoordinator, Logger};

#[derive(Clone, Copy)]
pub enum BoltKind {
    Fire,
}

#[derive(Clone, Copy)]
pub enum WeaponKind {
    Sword,
}

#[derive(Clone, Copy)]
pub enum AttackKind {
    Ranged(BoltKind),
    Melee(WeaponKind),
}

#[derive(Clone, Copy)]
pub struct AttackInfo {
    pub strength: u32,
    pub target: Point,
    pub kind: AttackKind,
}

impl AttackInfo {
    pub fn ranged_kind(&self) -> BoltKind {
        match self.kind {
            AttackKind::Ranged(kind) => kind,
            _ => panic!("Wrong type in ranged_kind"),
        }
    }

    pub fn melee_kind(&self) -> WeaponKind {
        match self.kind {
            AttackKind::Melee(kind) => kind,
            _ => panic!("Wrong type in melee_kind"),
        }
    }
}

#[derive(Component)]
pub struct AttackComponent {
    pub attack: AttackInfo,
}

impl AttackComponent {
    pub fn init(target: Point, strength: u32, kind: AttackKind) -> AttackComponent {
        AttackComponent {
            attack: AttackInfo { target, strength, kind },
        }
    }
}

pub fn combat_on_event(ecs: &mut World, kind: EventKind, target: &Entity) {
    match kind {
        EventKind::AnimationComplete(entity, effect) => match effect {
            PostAnimationEffect::ApplyBolt => {
                apply_bolt(ecs, &entity);
                ecs.delete_entity(*target).unwrap();
            }
            PostAnimationEffect::ApplyMelee => {
                apply_melee(ecs, &target);
            }
            PostAnimationEffect::None => {}
            _ => {}
        },
        _ => {}
    }
}

pub fn begin_bolt(ecs: &mut World, source: &Entity, target_position: Point, strength: u32, kind: BoltKind) {
    ecs.write_storage::<AttackComponent>()
        .insert(*source, AttackComponent::init(target_position, strength, AttackKind::Ranged(kind)))
        .unwrap();

    ecs.fire_event(EventKind::Bolt(*source), source);
}

pub fn start_bolt(ecs: &mut World, source: &Entity) -> Entity {
    let source_position = ecs.get_position(source);
    let attack = ecs.read_storage::<AttackComponent>().get(*source).unwrap().attack;

    let bolt = ecs
        .create_entity()
        .with(PositionComponent::init(source_position))
        .with(AttackComponent { attack })
        .build();

    ecs.write_storage::<AttackComponent>().remove(*source);

    bolt
}

fn apply_bolt(ecs: &mut World, bolt: &Entity) {
    let attack = {
        let attacks = ecs.read_storage::<AttackComponent>();
        attacks.get(*bolt).unwrap().attack
    };
    ecs.log(format!("Enemy was struck ({}) at ({},{})!", attack.strength, attack.target.x, attack.target.y).as_str());
}

pub fn begin_melee(ecs: &mut World, source: &Entity, target: Point, strength: u32, kind: WeaponKind) {
    ecs.write_storage::<AttackComponent>()
        .insert(*source, AttackComponent::init(target, strength, AttackKind::Melee(kind)))
        .unwrap();

    ecs.fire_event(EventKind::Melee(*source), &source);
}

fn apply_melee(ecs: &mut World, character: &Entity) {
    let attack = {
        let attacks = ecs.read_storage::<AttackComponent>();
        attacks.get(*character).unwrap().attack
    };
    ecs.log(format!("Enemy was struck ({}) in melee at ({},{})!", attack.strength, attack.target.x, attack.target.y).as_str());
}
