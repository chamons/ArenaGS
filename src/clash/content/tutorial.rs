// The ai macros can add "unnecessary" returns occationally
#![allow(clippy::needless_return)]

use std::collections::HashMap;

use specs::prelude::*;

use super::super::*;
use crate::try_behavior;

pub fn golem_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    m.insert(
        "Golem Punch",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::MeleeAttack(Damage::init(3), WeaponKind::Sword),
            Some(1),
            false,
        ),
    );

    m.insert(
        "Ground Slam",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::Field(FieldEffect::Damage(Damage::init(4), 1), FieldKind::Earth),
            Some(5),
            false,
        ),
    );
}

pub fn golem_action(ecs: &mut World, enemy: &Entity) {
    let distance = distance_to_player(ecs, enemy).unwrap_or(0);
    if distance <= 5 {
        if check_behavior_cooldown(ecs, enemy, "Ground Slam", 4) {
            try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Ground Slam"));
        }
    }
    try_behavior!(use_skill_at_player_if_in_range(ecs, enemy, "Golem Punch"));
    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));
    wait(ecs, *enemy);
}
