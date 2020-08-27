use std::collections::HashMap;

use specs::prelude::*;

use super::super::{get_skill, BoltKind, Damage, SkillEffect, SkillInfo, StatusInfo, StatusKind, TargetType};

pub fn get_weapon_skills() -> Vec<&'static str> {
    vec!["Aimed Shot"]
}

pub fn process_gunslinger_skill(ecs: &mut World, invoker: &Entity, name: &str) -> &'static SkillInfo {
    match name {
        "Aimed Shot" => {
            if ecs.has_status(invoker, StatusKind::FireAmmo) {
            } else if ecs.has_status(invoker, StatusKind::IceAmmo) {
            } else {
            }
            get_skill("Aimed Shot (Physical)")
        }
        _ => panic!("Unknown gunsliger skill {}", name),
    }
}

pub fn add_gunslinger_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    m.insert(
        "Aimed Shot",
        SkillInfo::init_with_distance("gun_06_b.png", TargetType::Enemy, SkillEffect::GunslingerAmmo, Some(6), true),
    );

    m.insert(
        "Aimed Shot (Physical)",
        SkillInfo::init_with_distance(
            "",
            TargetType::Enemy,
            SkillEffect::RangedAttack(Damage::init(5), BoltKind::Bullet),
            Some(6),
            true,
        ),
    );
}
