use specs::prelude::*;

use super::components::*;
use crate::clash::*;

fn player_can_act(ecs: &World) -> bool {
    let animations = ecs.read_storage::<AnimationComponent>();
    for a in (&animations).join() {
        match &a.animation {
            Animation::Position { .. } => {
                return false;
            }
            Animation::CharacterState { .. } => {}
        }
    }
    true
}

pub fn select_skill(ecs: &mut World, name: &str) {
    if !player_can_act(ecs) {
        return;
    }

    let target_required = get_target_for_skill(name);
    if target_required.is_none() {
        invoke_skill(ecs, name, None);
    } else {
        let mut state = ecs.write_resource::<BattleSceneStateComponent>();

        match target_required {
            TargetType::Enemy | TargetType::Tile => state.state = BattleSceneState::Targeting(BattleTargetSource::Skill(name.to_string()), target_required),
            TargetType::None => panic!("TargetType::None should not have reached here in select_skill"),
        }
    }
}

pub fn reset_targeting(ecs: &mut World) {
    let mut state = ecs.write_resource::<BattleSceneStateComponent>();
    state.state = BattleSceneState::Default();
}

pub fn select_skill_with_target(ecs: &mut World, name: &str, position: &Point) {
    if !player_can_act(ecs) {
        return;
    }

    // Selection has been made, drop out of targeting state
    reset_targeting(ecs);

    let target_required = get_target_for_skill(name);

    match target_required {
        TargetType::Enemy | TargetType::Tile => invoke_skill(ecs, name, Some(position)),
        TargetType::None => panic!("TargetType::None should not have reached select_skill_with_target"),
    }
}
