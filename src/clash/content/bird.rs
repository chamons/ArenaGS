use std::collections::HashMap;

use specs::prelude::*;

use super::super::*;
use crate::atlas::EasyECS;
use crate::{do_behavior, try_behavior};

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
    m.insert("Take Off", SkillInfo::init(None, TargetType::None, SkillEffect::Buff(StatusKind::Flying, 600)));
}

pub fn default_behavior(ecs: &mut World, enemy: &Entity) {
    if distance_to_player(ecs, enemy).unwrap_or(0) > 3 {
        try_behavior!(use_skill_if_in_range(ecs, enemy, "Feather Orb"));
    } else {
        try_behavior!(use_skill_if_in_range(ecs, enemy, "Wing Blast"));
    }
    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    wait(ecs, *enemy);
}

pub fn take_action(ecs: &mut World, enemy: &Entity) {
    let defenses = ecs.get_defenses(enemy);
    let phase = match defenses.health as f64 / defenses.max_health as f64 {
        x if x < 0.25 => 4,
        x if x < 0.5 => 3,
        x if x < 0.75 => 2,
        _ => 1,
    };

    if phase == 1 {
        do_behavior!(default_behavior(ecs, enemy));
    } else if phase == 2 {
        if ecs.has_status(enemy, StatusKind::Flying) {
        } else {
            try_behavior!(use_skill_with_cooldown(ecs, enemy, "Take Off", 4));
            do_behavior!(default_behavior(ecs, enemy));
        }
    }

    // Disappear for x seconds, drop bombs that explode after y seconds
    // Then repeat ground combat for z seconds
    wait(ecs, *enemy);
}
