use std::collections::HashMap;
use std::slice::from_ref;

use lazy_static::lazy_static;
use specs::prelude::*;

use super::{bolt, is_area_clear, melee, move_action, spend_time, BoltKind, Logger, Positions, WeaponKind, MOVE_ACTION_COST};
use crate::atlas::Point;

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
    RangedAttack(u32, BoltKind),
    MeleeAttack(u32, WeaponKind),
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

        if !match self.target {
            TargetType::Tile => is_area_clear(ecs, from_ref(&target), invoker),
            TargetType::Enemy => !is_area_clear(ecs, from_ref(&target), invoker),
            TargetType::None => false,
        } {
            return false;
        }

        if let Some(skill_range) = self.distance {
            if let Some(range_to_target) = initial.distance_to(target) {
                if range_to_target > skill_range {
                    return false;
                }
            }
        }

        if self.must_be_clear {
            if let Some(mut path) = initial.line_to(target) {
                // If we are targeting an enemy we can safely
                // ignore the last square, since we know that it must
                // have the target (from checks above)
                if self.target.is_enemy() {
                    path.pop();
                }
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
                SkillInfo::init_with_distance("", TargetType::Tile, SkillEffect::None, Some(2), false),
            );
            m.insert(
                "TestMove",
                SkillInfo::init_with_distance("", TargetType::Tile, SkillEffect::Move, Some(2), false),
            );
        }
        m.insert(
            "Dash",
            SkillInfo::init_with_distance("SpellBookPage09_39.png", TargetType::Tile, SkillEffect::Move, Some(3), true),
        );
        m.insert(
            "Fire Bolt",
            SkillInfo::init_with_distance(
                "SpellBook06_117.png",
                TargetType::Enemy,
                SkillEffect::RangedAttack(5, BoltKind::Fire),
                Some(15),
                true,
            ),
        );
        m.insert(
            "Slash",
            SkillInfo::init_with_distance(
                "SpellBook01_76.png",
                TargetType::Enemy,
                SkillEffect::MeleeAttack(5, WeaponKind::Sword),
                Some(1),
                true,
            ),
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

    let skill = get_skill(name);
    match skill.effect {
        SkillEffect::Move => {
            // Targeting only gives us a point, so clone their position to get size as well
            let position = ecs.get_position(invoker).move_to(target.unwrap());
            move_action(ecs, invoker, position);
        }
        SkillEffect::RangedAttack(strength, kind) => bolt(ecs, &invoker, target.unwrap(), strength, kind),
        SkillEffect::MeleeAttack(strength, kind) => melee(ecs, &invoker, target.unwrap(), strength, kind),
        SkillEffect::None => ecs.log(&format!("Invoking {}", name)),
    }

    spend_time(ecs, invoker, MOVE_ACTION_COST);
}

#[cfg(test)]
mod tests {
    use super::super::{create_world, wait_for_animations, Character, CharacterInfoComponent, Map, MapComponent, PositionComponent, TimeComponent};
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
        assert_eq!(0, ecs.read_storage::<TimeComponent>().grab(entity).ticks);
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
        ecs.insert(MapComponent::init(Map::init_empty()));
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

        let info = get_skill("TestWithRange");
        assert_eq!(true, info.is_good_target(&mut ecs, &entity, Point::init(2, 4)));
        assert_eq!(false, info.is_good_target(&mut ecs, &entity, Point::init(2, 5)));
        let info = SkillInfo::init("", TargetType::Tile, SkillEffect::None);
        assert_eq!(true, info.is_good_target(&mut ecs, &entity, Point::init(2, 5)));
    }

    #[test]
    fn skill_info_correct_target_kind() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.create_entity()
            .with(PositionComponent::init(SizedPoint::init(2, 3)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        let info = get_skill("TestWithRange");
        assert_eq!(true, info.is_good_target(&mut ecs, &entity, Point::init(2, 4)));
        assert_eq!(false, info.is_good_target(&mut ecs, &entity, Point::init(2, 3)));
    }

    #[test]
    fn skill_info_must_be_clear() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        let info = SkillInfo::init_with_distance("", TargetType::Tile, SkillEffect::None, Some(2), true);
        assert_eq!(true, info.is_good_target(&mut ecs, &entity, Point::init(2, 4)));
        ecs.create_entity()
            .with(PositionComponent::init(SizedPoint::init(2, 3)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();

        assert_eq!(false, info.is_good_target(&mut ecs, &entity, Point::init(2, 4)));
    }

    #[test]
    fn movement_effect() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        invoke_skill(&mut ecs, &entity, "TestMove", Some(Point::init(3, 3)));
        wait_for_animations(&mut ecs);

        assert_eq!(Point::init(3, 3), ecs.get_position(&entity).origin);
    }

    #[test]
    fn movement_effect_multi() {
        let mut ecs = create_world();
        let entity = ecs
            .create_entity()
            .with(TimeComponent::init(100))
            .with(PositionComponent::init(SizedPoint::init_multi(2, 2, 2, 1)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();
        ecs.insert(MapComponent::init(Map::init_empty()));

        invoke_skill(&mut ecs, &entity, "TestMove", Some(Point::init(3, 3)));
        wait_for_animations(&mut ecs);

        assert_eq!(Point::init(3, 3), ecs.get_position(&entity).origin);
    }
}
