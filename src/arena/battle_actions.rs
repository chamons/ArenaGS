use specs::prelude::*;

use super::components::*;
use super::AnimationComponent;
use crate::atlas::{EasyECS, Point};
use crate::clash::*;

pub fn has_animations_blocking(ecs: &World) -> bool {
    let animations = ecs.read_storage::<AnimationComponent>();
    (&animations).join().count() > 0
}

pub fn get_skill_name(ecs: &World, index: usize) -> Option<String> {
    let skills_component = ecs.read_storage::<SkillsComponent>();
    let skills = &skills_component.grab(find_player(&ecs)).skills;
    skills.get(index).map(|s| s.to_string())
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
            TargetType::Enemy | TargetType::Tile => set_state(ecs, BattleSceneState::Targeting(BattleTargetSource::Skill(name.to_string()))),
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
            let player = find_player(&ecs);
            if can_invoke_skill(ecs, &player, &skill, Some(*position)) {
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
