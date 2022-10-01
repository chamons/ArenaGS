use std::slice::from_ref;

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

use super::{find_character_at_location, find_position, is_area_clear_of_others, is_player_or_ally, Point};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SkillEffect {
    None,
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
    pub path_must_be_clear: bool,
}

impl Skill {
    pub fn new(name: &str, kind: SkillEffect, target: TargetType) -> Self {
        Skill {
            name: name.to_string(),
            kind,
            target,
            range: None,
            path_must_be_clear: false,
        }
    }

    pub fn with_range(mut self, range: u32) -> Skill {
        self.range = Some(range);
        self
    }

    pub fn path_must_be_clear(mut self) -> Skill {
        self.path_must_be_clear = true;
        self
    }

    pub fn show_trail(&self) -> bool {
        self.path_must_be_clear
    }
}

pub fn is_valid_target(world: &mut World, invoker: Entity, skill: &Skill, target: Point) -> bool {
    if !target.in_bounds() {
        return false;
    }

    let final_point_good = match skill.target {
        TargetType::Tile => is_area_clear_of_others(world, from_ref(&target), None),
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

    if skill.path_must_be_clear {
        if let Some(mut path) = find_position(world, invoker).unwrap().line_to(target) {
            path.pop();
            if !is_area_clear_of_others(world, &path, Some(invoker)) {
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

#[cfg(test)]
mod tests {
    use crate::core::{Character, Map, MapKind, Player, Position};

    use super::*;

    fn skill_test_world() -> (World, Entity) {
        let mut world = World::new();
        let first = world.spawn().insert(Character).insert(Player).insert(Position::new(2, 2)).id();
        world.spawn().insert(Character).insert(Position::new(3, 3));
        world.insert_resource(Map::empty(MapKind::Ashlands));
        (world, first)
    }

    #[test]
    fn valid_target_none() {
        let skill = Skill::new("Test", SkillEffect::None, TargetType::None);
        let (mut world, first) = skill_test_world();

        assert!(!is_valid_target(&mut world, first, &skill, Point::new(2, 2)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(2, 0)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(13, 13)));
    }

    #[test]
    fn valid_target_tile() {
        let skill = Skill::new("Test", SkillEffect::None, TargetType::Tile);
        let (mut world, first) = skill_test_world();

        assert!(!is_valid_target(&mut world, first, &skill, Point::new(2, 2)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(3, 3)));
        assert!(is_valid_target(&mut world, first, &skill, Point::new(0, 0)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(13, 13)));
    }

    #[test]
    fn valid_target_player() {
        let skill = Skill::new("Test", SkillEffect::None, TargetType::Player);
        let (mut world, first) = skill_test_world();

        assert!(is_valid_target(&mut world, first, &skill, Point::new(2, 2)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(3, 3)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(0, 0)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(13, 13)));
    }

    #[test]
    fn valid_target_enemy() {
        let skill = Skill::new("Test", SkillEffect::None, TargetType::Enemy);
        let (mut world, first) = skill_test_world();

        assert!(!is_valid_target(&mut world, first, &skill, Point::new(2, 2)));
        assert!(is_valid_target(&mut world, first, &skill, Point::new(3, 3)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(0, 0)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(13, 13)));
    }

    #[test]
    fn valid_target_any() {
        let skill = Skill::new("Test", SkillEffect::None, TargetType::Any);
        let (mut world, first) = skill_test_world();

        assert!(is_valid_target(&mut world, first, &skill, Point::new(2, 2)));
        assert!(is_valid_target(&mut world, first, &skill, Point::new(3, 3)));
        assert!(is_valid_target(&mut world, first, &skill, Point::new(0, 0)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(13, 13)));
    }

    #[test]
    fn valid_target_any_but_self() {
        let skill = Skill::new("Test", SkillEffect::None, TargetType::AnyoneButSelf);
        let (mut world, first) = skill_test_world();

        assert!(!is_valid_target(&mut world, first, &skill, Point::new(2, 2)));
        assert!(is_valid_target(&mut world, first, &skill, Point::new(3, 3)));
        assert!(is_valid_target(&mut world, first, &skill, Point::new(0, 0)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(13, 13)));
    }

    #[test]
    fn skill_range() {
        let skill = Skill::new("Test", SkillEffect::None, TargetType::Any).with_range(2);
        let (mut world, first) = skill_test_world();

        assert!(is_valid_target(&mut world, first, &skill, Point::new(3, 3)));
        assert!(is_valid_target(&mut world, first, &skill, Point::new(2, 4)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(2, 5)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(14, 5)));
    }

    #[test]
    fn skill_range_clear_path() {
        let skill = Skill::new("Test", SkillEffect::None, TargetType::Any).with_range(3).path_must_be_clear();
        let (mut world, first) = skill_test_world();

        assert!(is_valid_target(&mut world, first, &skill, Point::new(3, 3)));
        assert!(!is_valid_target(&mut world, first, &skill, Point::new(4, 4)));
    }
}
