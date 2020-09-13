// The ai macros can add "unnecessary" returns occationally
#![allow(clippy::needless_return)]

use std::collections::HashMap;

use specs::prelude::*;

use super::super::*;
use crate::try_behavior;

const TIDAL_SURGE_SIZE: u32 = 2;
const HEALING_MIST_RANGE: u32 = 5;

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
    wait(ecs, *enemy);
}

pub fn wind_elemental_action(ecs: &mut World, enemy: &Entity) {
    wait(ecs, *enemy);
}
pub fn earth_elemental_action(ecs: &mut World, enemy: &Entity) {
    wait(ecs, *enemy);
}
