use specs::prelude::*;

use super::components::*;
use crate::atlas::Point;
use crate::clash::*;

pub fn has_animations_blocking(ecs: &World) -> bool {
    let animations = ecs.read_storage::<AnimationComponent>();
    for a in (&animations).join() {
        match &a.animation {
            Animation::Position { .. } => {
                return true;
            }
            Animation::CharacterState { .. } => {}
        }
    }
    false
}

pub fn get_skill_name(ecs: &World, index: usize) -> Option<String> {
    let skills = ecs.read_storage::<SkillsComponent>();
    let player = find_player(&ecs).unwrap();
    let player_skill = skills.get(player).unwrap();
    if let Some(name) = player_skill.skills.get(index) {
        Some(name.to_string())
    } else {
        None
    }
}

pub fn select_skill(ecs: &mut World, name: &str) {
    if has_animations_blocking(ecs) {
        return;
    }

    let target_required = get_skill(name).target;
    if target_required.is_none() {
        player_use_skill(ecs, name, None);
    } else {
        match target_required {
            TargetType::Enemy | TargetType::Tile => set_state(ecs, BattleSceneState::Targeting(BattleTargetSource::Skill(name.to_string()), target_required)),
            TargetType::None => panic!("TargetType::None should not have reached here in select_skill"),
        }
    }
}

pub fn select_skill_with_target(ecs: &mut World, name: &str, position: &Point) {
    if has_animations_blocking(ecs) {
        return;
    }

    // Selection has been made, drop out of targeting state
    reset_state(ecs);

    let skill = get_skill(name);

    match skill.target {
        TargetType::Enemy | TargetType::Tile => {
            let player = find_player(&ecs).unwrap();
            let player_position = ecs.read_storage::<PositionComponent>().get(player).unwrap().position;
            // This unwrap is safe, as we should not have gotten here if we were invalid targeting state
            if skill.is_good_target(player_position, *position).unwrap() {
                player_use_skill(ecs, name, Some(*position));
            }
        }
        TargetType::None => panic!("TargetType::None should not have reached select_skill_with_target"),
    }
}

pub fn read_state(ecs: &World) -> BattleSceneState {
    ecs.read_resource::<BattleSceneStateComponent>().state.clone()
}

pub fn set_state(ecs: &mut World, state: BattleSceneState) {
    ecs.write_resource::<BattleSceneStateComponent>().state = state;
}

pub fn reset_state(ecs: &mut World) {
    let mut state = ecs.write_resource::<BattleSceneStateComponent>();
    state.state = BattleSceneState::Default();
}

pub fn move_action(ecs: &mut World, direction: Direction) {
    if has_animations_blocking(ecs) {
        return;
    }

    player_move(ecs, direction);
}
