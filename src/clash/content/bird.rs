// The ai macros can add "unnecessary" returns occasionally
#![allow(clippy::needless_return)]

use std::collections::HashMap;

use specs::prelude::*;

use super::super::*;
use crate::{do_behavior, try_behavior};

pub fn bird_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    // All skills will be boosted by default +1 skill_power on main bird
    m.add_skill(SkillInfo::init_with_distance(
        "Wing Blast",
        None,
        TargetType::Player,
        SkillEffect::RangedAttack(Damage::init(1), BoltKind::AirBullet),
        Some(2),
        true,
    ));
    m.add_skill(
        SkillInfo::init_with_distance(
            "Feather Orb",
            None,
            TargetType::Player,
            SkillEffect::Orb(Damage::init(3), OrbKind::Feather, 2, 12),
            Some(12),
            true,
        )
        .with_ammo(AmmoKind::Feathers, 1),
    );
    m.add_skill(
        SkillInfo::init_with_distance(
            "Tailwind",
            None,
            TargetType::Player,
            SkillEffect::Sequence(
                Box::new(SkillEffect::RangedAttack(
                    Damage::init(0).with_option(DamageOptions::KNOCKBACK),
                    BoltKind::AirBullet,
                )),
                Box::new(SkillEffect::ReloadSomeRandom(AmmoKind::Feathers, 3)),
            ),
            Some(6),
            true,
        )
        .with_cooldown(400),
    );
    m.add_skill(SkillInfo::init(
        "Explosive Eggs",
        None,
        TargetType::Tile,
        SkillEffect::Field(FieldEffect::Damage(Damage::init(3), 1), FieldKind::Fire),
    ));
    m.add_skill(SkillInfo::init("Take Off", None, TargetType::None, SkillEffect::Buff(StatusKind::Flying, 600)).with_cooldown(1000));
    m.add_skill(
        SkillInfo::init("Hatch", None, TargetType::None, SkillEffect::SpawnReplace(SpawnKind::BirdSpawn))
            .with_cooldown(400)
            .with_cooldown_spent(),
    );
    m.add_skill(
        SkillInfo::init("Throw Eggs", None, TargetType::Tile, SkillEffect::Spawn(SpawnKind::Egg))
            .with_ammo(AmmoKind::Eggs, 1)
            .with_cooldown(500),
    );
}

pub fn default_behavior(ecs: &mut World, enemy: Entity) {
    let distance = distance_to_player(ecs, enemy).unwrap_or(0);
    if distance > 7 {
        try_behavior!(move_towards_player(ecs, enemy));
    } else {
        try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Wing Blast"));
        try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Feather Orb"));
    }
    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    wait(ecs, enemy);
}

pub fn bird_action(ecs: &mut World, enemy: Entity) {
    let defenses = ecs.get_defenses(enemy);
    let phase = match defenses.health as f64 / defenses.max_health as f64 {
        x if x < 0.25 => 4,
        x if x < 0.5 => 3,
        x if x < 0.75 => 2,
        _ => 1,
    };

    if phase == 1 {
        try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Tailwind"));
        do_behavior!(default_behavior(ecs, enemy));
    } else if phase == 2 {
        if ecs.has_status(enemy, StatusKind::Flying) {
            try_behavior!(use_skill_with_random_target(ecs, enemy, "Explosive Eggs", 4));
        } else {
            try_behavior!(use_skill(ecs, enemy, "Take Off"));
            try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Tailwind"));
            do_behavior!(default_behavior(ecs, enemy));
        }
    } else if phase == 3 {
        try_behavior!(use_skill_with_random_target(ecs, enemy, "Throw Eggs", 6));
        do_behavior!(default_behavior(ecs, enemy));
    } else if phase == 4 {
        if ecs.has_status(enemy, StatusKind::Flying) {
            if coin_flip(ecs) {
                try_behavior!(use_skill_with_random_target(ecs, enemy, "Explosive Eggs", 4));
            }
            try_behavior!(use_skill_with_random_target(ecs, enemy, "Throw Eggs", 8));
        } else {
            try_behavior!(use_skill(ecs, enemy, "Take Off"));
            do_behavior!(default_behavior(ecs, enemy));
        }
    }
    wait(ecs, enemy);
}

pub fn bird_add_action(ecs: &mut World, enemy: Entity) {
    do_behavior!(default_behavior(ecs, enemy));
}

pub fn egg_action(ecs: &mut World, enemy: Entity) {
    try_behavior!(use_skill(ecs, enemy, "Hatch"));
    wait(ecs, enemy);
}
