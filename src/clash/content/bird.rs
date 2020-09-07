use std::collections::HashMap;

use specs::prelude::*;

use super::super::*;
use crate::try_behavior;

pub fn bird_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    m.insert(
        "Wing Blast",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::RangedAttack(Damage::init(2), BoltKind::Bullet),
            Some(3),
            true,
        ),
    );
    m.insert(
        "Feather Orb",
        SkillInfo::init_with_distance(None, TargetType::Player, SkillEffect::Orb(Damage::init(4), OrbKind::Feather, 2), Some(12), true),
    );
}

pub fn take_action(ecs: &mut World, enemy: &Entity) {
    if distance_to_player(ecs, enemy).unwrap_or(0) > 3 {
        try_behavior!(use_skill_if_in_range(ecs, enemy, "Feather Orb"));
    } else {
        try_behavior!(use_skill_if_in_range(ecs, enemy, "Wing Blast"));
    }
    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));

    wait(ecs, *enemy);
}
