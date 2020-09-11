use specs::prelude::*;

use super::components::*;
use super::AnimationComponent;
use crate::atlas::{Direction, Point};
use crate::clash::*;

pub enum BattleActionRequest {
    Move(Direction),
    SelectSkill(String),
    TargetSkill(String, Point),
}

pub fn request_action(ecs: &mut World, request: BattleActionRequest) {
    if has_animations_blocking(ecs) {
        return;
    }
    match request {
        BattleActionRequest::Move(direction) => move_action(ecs, direction),
        BattleActionRequest::SelectSkill(name) => select_skill(ecs, &name),
        BattleActionRequest::TargetSkill(name, target) => select_skill_with_target(ecs, &name, &target),
    }
}

// Prevents actions when animations in progress. actions::can_act handles world state
pub fn has_animations_blocking(ecs: &World) -> bool {
    let animations = ecs.read_storage::<AnimationComponent>();
    (&animations).join().count() > 0
}

fn select_skill(ecs: &mut World, name: &str) {
    let skill = get_skill(name);

    match skill.is_usable(ecs, &find_player(&ecs)) {
        UsableResults::Usable => {}
        _ => return,
    }

    let target_required = skill.target;
    if target_required.is_none() {
        player_use_skill(ecs, name, None);
    } else {
        match target_required {
            TargetType::Any | TargetType::Enemy | TargetType::Tile => {
                set_action_state(ecs, BattleSceneState::Targeting(BattleTargetSource::Skill(name.to_string())))
            }
            TargetType::Player => panic!("TargetType::Player should not have reached here in select_skill"),
            TargetType::None => panic!("TargetType::None should not have reached here in select_skill"),
        }
    }
}

fn select_skill_with_target(ecs: &mut World, name: &str, position: &Point) {
    // Selection has been made, drop out of targeting state
    reset_action_state(ecs);

    let skill = get_skill(name);

    match skill.target {
        TargetType::Enemy | TargetType::Tile | TargetType::Any => {
            let player = find_player(&ecs);
            if can_invoke_skill(ecs, &player, &skill, Some(*position)) {
                player_use_skill(ecs, name, Some(*position));
            }
        }
        TargetType::Player => panic!("TargetType::Player should not have reached select_skill_with_target"),
        TargetType::None => panic!("TargetType::None should not have reached select_skill_with_target"),
    }
}

pub fn read_action_state(ecs: &World) -> BattleSceneState {
    ecs.read_resource::<BattleSceneStateComponent>().state.clone()
}

pub fn set_action_state(ecs: &mut World, state: BattleSceneState) {
    ecs.write_resource::<BattleSceneStateComponent>().state = state;
}

pub fn reset_action_state(ecs: &mut World) {
    let mut state = ecs.write_resource::<BattleSceneStateComponent>();
    state.state = BattleSceneState::Default();
}

fn move_action(ecs: &mut World, direction: Direction) {
    player_move(ecs, direction);
}
