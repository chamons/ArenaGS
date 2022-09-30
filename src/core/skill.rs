use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

use super::{find_character_at_location, is_area_clear_of_others, is_player_or_ally, Point};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SkillEffect {
    Move,
    RangedAttack,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum TargetType {
    None,
    Tile,
    Player,
    Enemy,
    Any,
    AnyoneButSelf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Skill {
    pub name: String,
    pub kind: SkillEffect,
    pub target: TargetType,
    pub range: Option<u32>,
    pub must_be_clear: bool,
}

impl Skill {
    pub fn new(name: &str, kind: SkillEffect, target: TargetType) -> Self {
        Skill {
            name: name.to_string(),
            kind,
            target,
            range: None,
            must_be_clear: false,
        }
    }

    pub fn with_range(mut self, range: u32) -> Skill {
        self.range = Some(range);
        self
    }

    pub fn must_be_clear(mut self) -> Skill {
        self.must_be_clear = true;
        self
    }
}

pub fn is_valid_target(world: &mut World, invoker: Entity, skill: &Skill, target: Option<Point>) -> bool {
    let final_point_good = match skill.target {
        TargetType::Tile => is_area_clear_of_others(ecs, from_ref(&target), invoker),
        TargetType::Enemy => !is_area_clear_of_others(ecs, from_ref(&target), invoker),
        TargetType::Player => {
            if let Some(potential_target) = find_character_at_location(ecs, target) {
                is_player_or_ally(ecs, potential_target)
            } else {
                false
            }
        }
        TargetType::AnyoneButSelf => {
            if let Some(initial) = ecs.read_storage::<PositionComponent>().get(invoker) {
                !initial.position.contains_point(&target)
            } else {
                true
            }
        }
        TargetType::Any => true,
        TargetType::None => false,
    };

    if !final_point_good {
        return false;
    }

    if !in_possible_skill_range(ecs, invoker, skill, target) {
        return false;
    }

    true
}

pub fn in_possible_skill_range(ecs: &World, invoker: Entity, skill: &SkillInfo, target: Point) -> bool {
    if let Some(skill_range) = skill.range {
        if let Some(range_to_target) = ecs.get_position(invoker).distance_to(target) {
            if range_to_target > skill_range {
                return false;
            }
        }
    }

    if skill.must_be_clear {
        if let Some(mut path) = ecs.get_position(invoker).line_to(target) {
            // If we are targeting an enemy/player we can safely
            // ignore the last square, since we know that it must
            // have the target (from checks above)
            if skill.target.is_enemy() || skill.target.is_player() {
                path.pop();
            }
            if !is_area_clear_of_others(ecs, &path, invoker) {
                return false;
            }
        }
    }
    true
}

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Skills {
    pub skills: Vec<Skill>,
}

impl Skills {
    pub fn new(skills: &[Skill]) -> Self {
        Skills { skills: Vec::from(skills) }
    }
}
