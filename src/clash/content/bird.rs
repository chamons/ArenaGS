use std::collections::HashMap;

use specs::prelude::*;

use super::super::*;

pub fn bird_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    m.insert(
        "Feather Blast",
        SkillInfo::init_with_distance(
            None,
            TargetType::Player,
            SkillEffect::RangedAttack(Damage::init(2), BoltKind::Bullet),
            Some(7),
            true,
        ),
    );
}

pub fn take_action(ecs: &mut World, enemy: &Entity, phase: u32) {
    let current_position = ecs.get_position(enemy);
    let player_position = ecs.get_position(&find_player(ecs));
    if let Some((_, target_point, distance)) = current_position.distance_to_multi_with_endpoints(player_position) {
        let skill = get_skill("Feather Blast");
        if distance <= skill.range.unwrap() {
            if is_good_target(ecs, enemy, skill, target_point) {
                invoke_skill(ecs, enemy, "Feather Blast", Some(target_point));
                return;
            }
        }
    }

    wait(ecs, *enemy);
}
