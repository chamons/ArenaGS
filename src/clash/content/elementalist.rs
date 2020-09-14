// The ai macros can add "unnecessary" returns occationally
#![allow(clippy::needless_return)]

use std::collections::HashMap;

use specs::prelude::*;

use super::super::*;
use crate::try_behavior;

const TIDAL_SURGE_SIZE: u32 = 2;
const HEALING_MIST_RANGE: u32 = 5;
const MAGMA_ERUPTION_RANGE: u32 = 7;

pub fn elementalist_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    m.insert(
        "Tidal Surge",
        SkillInfo::init_with_distance(
            None,
            TargetType::AnyoneButSelf,
            SkillEffect::ConeAttack(Damage::init(3).with_option(DamageOptions::KNOCKBACK), ConeKind::Water, TIDAL_SURGE_SIZE),
            Some(1),
            false,
        ),
    );

    m.insert(
        "Ice Shard",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::RangedAttack(Damage::init(2), BoltKind::Water),
            Some(2),
            false,
        ),
    );

    m.insert(
        "Healing Mist",
        SkillInfo::init_with_distance(
            None,
            TargetType::Any,
            SkillEffect::Buff(StatusKind::Regen, 400),
            Some(HEALING_MIST_RANGE),
            false,
        ),
    );

    m.insert(
        "Magma Eruption",
        SkillInfo::init_with_distance(
            None,
            TargetType::Any,
            SkillEffect::Field(
                FieldEffect::SustainedDamage(Damage::init(1).with_option(DamageOptions::PIERCE_DEFENSES), 6),
                FieldKind::Fire,
            ),
            Some(MAGMA_ERUPTION_RANGE),
            false,
        ),
    );

    m.insert(
        "Lava Bolt",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::RangedAttack(Damage::init(3), BoltKind::Fire),
            Some(4),
            false,
        ),
    );

    m.insert(
        "Lightning Surge",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::RangedAttack(Damage::init(3), BoltKind::Lightning),
            Some(4),
            false,
        ),
    );

    m.insert(
        "Hailstone",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::Field(FieldEffect::Damage(Damage::init(4), 1), FieldKind::Hail),
            Some(8),
            false,
        ),
    );

    m.insert(
        "Earthen Rage",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::ChargeAttack(Damage::init(3).with_option(DamageOptions::KNOCKBACK), WeaponKind::Sword),
            Some(5),
            false,
        ),
    );

    m.insert(
        "Rock Slide",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::ChargeAttack(Damage::init(2), WeaponKind::Sword),
            Some(3),
            false,
        ),
    );

    m.insert(
        "Pummel",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::MeleeAttack(Damage::init(3), WeaponKind::Sword),
            Some(1),
            false,
        ),
    );
}

pub fn elementalist_action(ecs: &mut World, enemy: &Entity) {
    wait(ecs, *enemy);
}
pub fn water_elemental_action(ecs: &mut World, enemy: &Entity) {
    let current_position = ecs.get_position(enemy);
    let distance = distance_to_player(ecs, enemy).unwrap_or(0);
    if distance <= 3 {
        if let Some(cone_target) = check_for_cone_striking_player(ecs, enemy, TIDAL_SURGE_SIZE) {
            if check_behavior_cooldown(ecs, enemy, "Tidal Surge", 2) {
                try_behavior!(use_skill_at_position(ecs, enemy, "Tidal Surge", &cone_target));
            }
        }
        try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Ice Shard"));
    }

    if let Some(target) = any_ally_without_buff_in_range(ecs, enemy, StatusKind::Regen, HEALING_MIST_RANGE) {
        try_behavior!(use_skill_at_position(
            ecs,
            enemy,
            "Healing Mist",
            &ecs.get_position(&target).nearest_point_to(current_position)
        ));
    }

    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    wait(ecs, *enemy);
}

pub fn fire_elemental_action(ecs: &mut World, enemy: &Entity) {
    try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Lava Bolt"));

    let player_position = ecs.get_position(&find_player(ecs));
    if find_field_at_location(ecs, &player_position).is_none() {
        if check_behavior_cooldown(ecs, enemy, "Magma Eruption", 2) {
            try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Magma Eruption"));
        }
    }

    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    wait(ecs, *enemy);
}

pub fn wind_elemental_action(ecs: &mut World, enemy: &Entity) {
    try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Lightning Surge"));

    let player_position = ecs.get_position(&find_player(ecs));
    if find_field_at_location(ecs, &player_position).is_none() {
        if check_behavior_cooldown(ecs, enemy, "Hailstone", 2) {
            try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Hailstone"));
        }
    }

    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    wait(ecs, *enemy);
}
pub fn earth_elemental_action(ecs: &mut World, enemy: &Entity) {
    let distance = distance_to_player(ecs, enemy).unwrap_or(0);
    if distance < 6 {
        if check_behavior_cooldown(ecs, enemy, "Earthen Rage", 4) {
            try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Earthen Rage"));
        }
    }
    if distance < 4 {
        if check_behavior_cooldown(ecs, enemy, "Rock Slide", 3) {
            try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Rock Slide"));
        }
    }
    try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Pummel"));
    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    wait(ecs, *enemy);
}
