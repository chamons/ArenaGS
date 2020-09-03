use std::collections::HashMap;

use specs::prelude::*;

use super::super::*;

pub fn bird_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    m.insert(
        "Feather Blast",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::RangedAttack(Damage::init(2), BoltKind::Smoke),
            Some(7),
            true,
        ),
    );
}

pub fn take_action(ecs: &mut World, enemy: &Entity, phase: u32) {
    wait(ecs, *enemy);
}
