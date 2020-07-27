use std::collections::HashMap;

use lazy_static::lazy_static;
use specs::prelude::*;

use super::Point;
use super::{spend_time, Logger, MOVE_ACTION_COST};

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
    Move { distance: i32 },
}

pub struct SkillInfo {
    pub image: &'static str,
    pub target: TargetType,
    pub effect: SkillEffect,
}

impl SkillInfo {
    pub fn init(image: &'static str, target: TargetType, effect: SkillEffect) -> SkillInfo {
        SkillInfo { image, target, effect }
    }

    pub fn show_trail(&self) -> bool {
        false
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
        }
        m.insert(
            "Dash",
            SkillInfo::init("SpellBookPage09_39.png", TargetType::Tile, SkillEffect::Move { distance: 3 }),
        );
        m
    };
}

pub fn get_skill(name: &str) -> &SkillInfo {
    SKILLS.get(name).unwrap()
}

fn assert_correct_targeting(name: &str, target: Option<Point>) {
    let requires_point = match get_skill(name).target {
        TargetType::None => false,
        TargetType::Tile => true,
        TargetType::Enemy => true,
    };

    if requires_point != target.is_some() {
        panic!("invoke_skill for {} called with wrong targeting param state.", name);
    }
}

pub fn invoke_skill(ecs: &mut World, invoker: &Entity, name: &str, target: Option<Point>) {
    assert_correct_targeting(name, target);
    ecs.log(&format!("Invoking {}", name));
    spend_time(ecs, invoker, MOVE_ACTION_COST);
}

#[cfg(test)]
mod tests {
    use super::super::{create_world, TimeComponent};
    use super::*;

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
}
