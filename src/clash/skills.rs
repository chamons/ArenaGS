use std::collections::HashMap;

use lazy_static::lazy_static;
use specs::prelude::*;

use super::{is_area_clear, spend_time, Logger, Positions, MOVE_ACTION_COST};
use crate::atlas::{Point, SizedPoint};

#[allow(dead_code)]
#[derive(is_enum_variant, Clone, Copy)]
pub enum TargetType {
    None,
    Tile,
    Enemy,
}

#[allow(dead_code)]
pub enum SkillEffect {
    None,
    Move,
}

pub struct SkillInfo {
    pub image: &'static str,
    pub target: TargetType,
    pub effect: SkillEffect,
    pub distance: Option<u32>,
    pub must_be_clear: bool,
}

impl SkillInfo {
    #[allow(dead_code)]
    pub fn init(image: &'static str, target: TargetType, effect: SkillEffect) -> SkillInfo {
        SkillInfo {
            image,
            target,
            effect,
            distance: None,
            must_be_clear: false,
        }
    }

    pub fn init_with_distance(image: &'static str, target: TargetType, effect: SkillEffect, distance: Option<u32>, must_be_clear: bool) -> SkillInfo {
        SkillInfo {
            image,
            target,
            effect,
            distance,
            must_be_clear,
        }
    }

    pub fn show_trail(&self) -> bool {
        self.must_be_clear
    }

    pub fn is_good_target(&self, ecs: &World, invoker: &Entity, target: Point) -> bool {
        let initial = ecs.get_position(invoker);

        if let Some(skill_range) = self.distance {
            if let Some(range_to_target) = initial.distance_to(target) {
                if range_to_target > skill_range {
                    return false;
                }
            }
        }

        if self.must_be_clear {
            if let Some(path) = initial.line_to(target) {
                if !is_area_clear(ecs, &path, invoker) {
                    return false;
                }
            }
        }
        true
    }
}

lazy_static! {
    static ref SKILLS: HashMap<&'static str, SkillInfo> = {
        let mut m = HashMap::new();
        #[cfg(test)]
        {
            m.insert("TestNone", SkillInfo::init("", TargetType::None, SkillEffect::None));
            m.insert("TestTile", SkillInfo::init("", TargetType::Tile, SkillEffect::None));
            m.insert("TestEnemy", SkillInfo::init("", TargetType::Enemy, SkillEffect::None));
            m.insert(
                "TestWithRange",
                SkillInfo::init_with_distance("", TargetType::Tile, SkillEffect::Move, Some(2), false),
            );
        }
        m.insert(
            "Dash",
            SkillInfo::init_with_distance("SpellBookPage09_39.png", TargetType::Tile, SkillEffect::Move, Some(3), true),
        );
        m
    };
}

pub fn get_skill(name: &str) -> &'static SkillInfo {
    SKILLS.get(name).unwrap()
}

fn assert_correct_targeting(ecs: &mut World, invoker: &Entity, name: &str, target: Option<Point>) {
    let skill = get_skill(name);

    let requires_point = match skill.target {
        TargetType::None => false,
        TargetType::Tile => true,
        TargetType::Enemy => true,
    };

    if requires_point != target.is_some() {
        panic!("invoke_skill for {} called with wrong targeting param state.", name);
    }

    if let Some(target) = target {
        assert!(skill.is_good_target(ecs, invoker, target));
    }
}

pub fn invoke_skill(ecs: &mut World, invoker: &Entity, name: &str, target: Option<Point>) {
    assert_correct_targeting(ecs, invoker, name, target);
    ecs.log(&format!("Invoking {}", name));
    spend_time(ecs, invoker, MOVE_ACTION_COST);
}

#[cfg(test)]
mod tests {
    use super::super::{create_world, Map, MapComponent, PositionComponent, TimeComponent, Character, CharacterInfoComponent};
    use super::*;
    use crate::atlas::SizedPoint;

    #[test]
    #[should_panic]
    fn panic_if_wrong_targeting() {
        let mut ecs = create_world();
        let entity = ecs.create_entity().with(TimeComponent::init(100)).build();
        invoke_skill(&mut ecs, &entity, "TestNone", Some(Point::init(2, 2)));
    }

    #[test]
    #[should_panic]
    fn panic_if_missing_targeting() {
        let mut ecs = create_world();
        let entity = ecs.create_entity().with(TimeComponent::init(100)).build();
        invoke_skill(&mut ecs, &entity, "TestTile", None);
    }

    #[test]
    fn invoker_spend_time() {
        let mut ecs = create_world();
        let entity = ecs.create_entity().with(TimeComponent::init(100)).build();
        invoke_skill(&mut ecs, &entity, "TestNone", None);
        assert_eq!(0, ecs.read_storage::<TimeComponent>().get(entity).unwrap().ticks);
    }

    #[test]
    #[should_panic]
    fn target_must_be_in_range() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .build();
        invoke_skill(&mut ecs, &entity, "TestWithRange", Some(Point::init(2, 5)));
    }

    #[test]
    fn target_in_range() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .build();
        invoke_skill(&mut ecs, &entity, "TestWithRange", Some(Point::init(2, 4)));
    }

    #[test]
    fn skill_info_range() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        let info = SkillInfo::init_with_distance("", TargetType::Tile, SkillEffect::Move, Some(2), false);
        assert_eq!(true, info.is_good_target(&mut ecs, &entity, Point::init(2, 4)));
        assert_eq!(false, info.is_good_target(&mut ecs, &entity, Point::init(2, 5)));
        let info = SkillInfo::init("", TargetType::Tile, SkillEffect::None);
        assert_eq!(true, info.is_good_target(&mut ecs, &entity, Point::init(2, 5)));
    }

    #[test]
    fn skill_info_clear() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        let info = SkillInfo::init_with_distance("", TargetType::Tile, SkillEffect::Move, Some(2), true);
        assert_eq!(true, info.is_good_target(&mut ecs, &entity, Point::init(2, 4)));
        ecs
            .create_entity()
            .with(PositionComponent::init(SizedPoint::init(2, 3)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();

        assert_eq!(false, info.is_good_target(&mut ecs, &entity, Point::init(2, 4)));
    }
}
