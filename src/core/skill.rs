use std::slice::from_ref;

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

use super::{find_character_at_location, find_position, is_area_clear_of_others, is_player_or_ally, Point, Position};

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

    pub fn show_trail(&self) -> bool {
        self.must_be_clear
    }
}

pub fn is_valid_target(world: &mut World, invoker: Entity, skill: &Skill, target: Point) -> bool {
    let final_point_good = match skill.target {
        TargetType::Tile => is_area_clear_of_others(world, from_ref(&target), invoker),
        TargetType::Enemy => {
            if let Some(potential_target) = find_character_at_location(world, target) {
                !is_player_or_ally(world, potential_target)
            } else {
                false
            }
        }
        TargetType::Player => {
            if let Some(potential_target) = find_character_at_location(world, target) {
                is_player_or_ally(world, potential_target)
            } else {
                false
            }
        }
        TargetType::AnyoneButSelf => {
            if let Some(initial) = find_position(world, invoker) {
                !initial.contains_point(&target)
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

    if !in_possible_skill_range(world, invoker, skill, target) {
        return false;
    }

    true
}

pub fn in_possible_skill_range(world: &mut World, invoker: Entity, skill: &Skill, target: Point) -> bool {
    if let Some(skill_range) = skill.range {
        if let Some(range_to_target) = find_position(world, invoker).unwrap().distance_to(target) {
            if range_to_target > skill_range {
                return false;
            }
        }
    }

    if skill.must_be_clear {
        if let Some(mut path) = find_position(world, invoker).unwrap().line_to(target) {
            // If we are targeting an enemy/player we can safely
            // ignore the last square, since we know that it must
            // have the target (from checks above)
            if matches!(skill.target, TargetType::Enemy | TargetType::Player) {
                path.pop();
            }
            if !is_area_clear_of_others(world, &path, invoker) {
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
