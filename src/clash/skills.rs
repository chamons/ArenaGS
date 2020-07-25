use specs::prelude::*;

use super::Point;
use super::{spend_time, MOVE_ACTION_COST};
use crate::atlas::Logger;

#[allow(dead_code)]
#[derive(is_enum_variant, Clone)]
pub enum TargetType {
    None,
    Tile,
    Enemy,
}

pub fn get_target_for_skill(_name: &str) -> TargetType {
    #[cfg(test)]
    {
        if let Some(test_match) = match _name {
            "TestNone" => Some(TargetType::None),
            "TestTile" => Some(TargetType::Tile),
            "TestEnemy" => Some(TargetType::Enemy),
            _ => None,
        } {
            return test_match;
        }
    }

    TargetType::Enemy
}

fn assert_correct_targeting(name: &str, target: Option<Point>) {
    let requires_point = match get_target_for_skill(name) {
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
    use super::super::create_world;
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
