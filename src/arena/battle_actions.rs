use specs::prelude::*;

use super::components::*;
use super::AnimationComponent;
use crate::atlas::{Direction, EasyECS, Point};
use crate::clash::*;

pub enum BattleActionRequest {
    None
    SelectSkill(String),
    Move(Direction),
}

// Prevents actions when animations in progress. actions::can_act handles world state
pub fn has_animations_blocking(ecs: &World) -> bool {
    let animations = ecs.read_storage::<AnimationComponent>();
    (&animations).join().count() > 0
}

pub fn get_skill_name(ecs: &World, index: usize) -> Option<String> {
    let skills_component = ecs.read_storage::<SkillsComponent>();
    let skills = &skills_component.grab(find_player(&ecs)).skills;
    skills.get(index).map(|s| get_current_skill(ecs, s))
}

// Some skills have an alternate when not usable (such as reload)
pub fn get_current_skill(ecs: &World, skill_name: &str) -> String {
    let skill = get_skill(skill_name);

    match skill.is_usable(ecs, &find_player(&ecs)) {
        UsableResults::LacksAmmo if skill.alternate.is_some() => skill.alternate.as_ref().unwrap().to_string(),
        _ => skill_name.to_string(),
    }
}

pub fn select_skill(ecs: &mut World, name: &str) {
    if has_animations_blocking(ecs) {
        return;
    }

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
            TargetType::Any | TargetType::Enemy | TargetType::Tile => set_state(ecs, BattleSceneState::Targeting(BattleTargetSource::Skill(name.to_string()))),
            TargetType::Player => panic!("TargetType::Player should not have reached here in select_skill"),
            TargetType::None => panic!("TargetType::None should not have reached here in select_skill"),
        }
    }
}

pub fn select_skill_with_target(ecs: &mut World, name: &str, position: &Point) {
    // Selection has been made, drop out of targeting state
    reset_state(ecs);

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
