// The ai macros can add "unnecessary" returns occationally
#![allow(clippy::needless_return)]

use std::collections::HashMap;

use specs::prelude::*;

use super::super::*;
use crate::{do_behavior, try_behavior, try_behavior_and_if};

pub fn bird_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    // All skills will be boosted by default +1 skill_power on main bird
    m.insert(
        "Wing Blast",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::RangedAttack(Damage::init(1), BoltKind::AirBullet),
            Some(2),
            true,
        ),
    );
    m.insert(
        "Feather Orb",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::Orb(Damage::init(3), OrbKind::Feather, 2, 12),
            Some(12),
            true,
        ),
    );
    m.insert(
        "Tailwind",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::RangedAttack(Damage::init(0).with_option(DamageOptions::KNOCKBACK), BoltKind::AirBullet),
            Some(6),
            true,
        ),
    );
    m.insert(
        "Explosive Eggs",
        SkillInfo::init(
            None,
            TargetType::Tile,
            SkillEffect::Field(FieldEffect::Damage(Damage::init(3)), FieldKind::Fire, 1),
        ),
    );
    m.insert("Take Off", SkillInfo::init(None, TargetType::None, SkillEffect::Buff(StatusKind::Flying, 600)));
    m.insert(
        "Hatch",
        SkillInfo::init(None, TargetType::None, SkillEffect::SpawnReplace(SpawnKind::BirdSpawn)),
    );
    m.insert("Throw Eggs", SkillInfo::init(None, TargetType::Tile, SkillEffect::Spawn(SpawnKind::Egg)));
}

pub fn default_behavior(ecs: &mut World, enemy: &Entity) {
    let distance = distance_to_player(ecs, enemy).unwrap_or(0);
    if distance > 7 {
        try_behavior!(move_towards_player(ecs, enemy));
    } else {
        try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Wing Blast"));
        // Flip between 2 and 3 shots before pausing a round
        if check_behavior_ammo_calculate(ecs, enemy, "Feather-Ammo", |ecs| flip_value(ecs, enemy, "Feather-Ammo-Amount", 3, 2)) {
            try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Feather Orb"));
        }
    }
    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    wait(ecs, *enemy);
}

pub fn bird_action(ecs: &mut World, enemy: &Entity) {
    let defenses = ecs.get_defenses(enemy);
    let phase = match defenses.health as f64 / defenses.max_health as f64 {
        x if x < 0.25 => 4,
        x if x < 0.5 => 3,
        x if x < 0.75 => 2,
        _ => 1,
    };

    if phase == 1 {
        try_behavior!(use_player_target_skill_with_cooldown(ecs, enemy, "Tailwind", 4));
        do_behavior!(default_behavior(ecs, enemy));
    } else if phase == 2 {
        if ecs.has_status(enemy, StatusKind::Flying) {
            if check_behavior_cooldown(ecs, enemy, "Bombing Run", 0) {
                try_behavior!(use_skill_with_random_target(ecs, enemy, "Explosive Eggs", 4));
            }
        } else {
            try_behavior_and_if!(
                use_no_target_skill_with_cooldown(ecs, enemy, "Take Off", 4),
                set_behavior_value(ecs, enemy, "Bombing Run", 1)
            );
            try_behavior!(use_player_target_skill_with_cooldown(ecs, enemy, "Tailwind", 4));
            do_behavior!(default_behavior(ecs, enemy));
        }
    } else if phase == 3 {
        if check_behavior_cooldown(ecs, enemy, "Throw Eggs", 3) {
            try_behavior!(use_skill_with_random_target(ecs, enemy, "Throw Eggs", 6));
        }
        do_behavior!(default_behavior(ecs, enemy));
    } else if phase == 4 {
        if ecs.has_status(enemy, StatusKind::Flying) {
            if check_behavior_cooldown(ecs, enemy, "Bombing Run", 0) {
                if coin_flip(ecs) {
                    try_behavior!(use_skill_with_random_target(ecs, enemy, "Explosive Eggs", 4));
                } else {
                    try_behavior!(use_skill_with_random_target(ecs, enemy, "Throw Eggs", 8));
                }
            }
        } else {
            try_behavior_and_if!(
                use_no_target_skill_with_cooldown(ecs, enemy, "Take Off", 4),
                set_behavior_value(ecs, enemy, "Bombing Run", 1)
            );
            do_behavior!(default_behavior(ecs, enemy));
        }
    }
    wait(ecs, *enemy);
}

pub fn bird_add_action(ecs: &mut World, enemy: &Entity) {
    do_behavior!(default_behavior(ecs, enemy));
}

pub fn egg_action(ecs: &mut World, enemy: &Entity) {
    try_behavior!(use_no_target_skill_with_cooldown(ecs, enemy, "Hatch", 4));
    wait(ecs, *enemy);
}
