// The ai macros can add "unnecessary" returns occationally
#![allow(clippy::needless_return)]

use specs::prelude::*;

use super::super::*;
use crate::try_behavior;

pub fn golem_skills(m: &mut SkillsResource) {
    m.add(SkillInfo::init_with_distance(
        "Golem Punch",
        None,
        TargetType::Player,
        SkillEffect::MeleeAttack(Damage::init(3, DamageElement::PHYSICAL), WeaponKind::Sword),
        Some(1),
        false,
    ));

    m.add(
        SkillInfo::init_with_distance(
            "Ground Slam",
            None,
            TargetType::Player,
            SkillEffect::Field(FieldEffect::Damage(Damage::init(4, DamageElement::PHYSICAL), 1), FieldKind::Earth),
            Some(5),
            false,
        )
        .with_cooldown(300),
    );
}

pub fn golem_action(ecs: &mut World, enemy: Entity) {
    try_behavior!(use_skill_at_any_enemy_if_in_range(ecs, enemy, "Golem Punch"));
    try_behavior!(use_skill_at_any_enemy_if_in_range(ecs, enemy, "Ground Slam"));
    try_behavior!(move_towards_player(ecs, enemy));
    wait(ecs, enemy);
}
