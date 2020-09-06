use std::collections::HashMap;

use specs::prelude::*;

use super::super::*;
use crate::try_behavior;

pub fn bird_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    m.insert(
        "Feather Blast",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::RangedAttack(Damage::init(2), BoltKind::Bullet),
            Some(3),
            true,
        ),
    );
}

pub fn take_action(ecs: &mut World, enemy: &Entity) {
    try_behavior!(use_skill_if_in_range(ecs, enemy, "Feather Blast"));
    try_behavior!(move_towards_player(ecs, enemy));
    try_behavior!(move_randomly(ecs, enemy));

    wait(ecs, *enemy);
}
