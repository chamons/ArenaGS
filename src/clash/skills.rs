use specs::prelude::*;

use super::Point;
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

fn assert_correct_targeting(name: &str, target: Option<&Point>) {
    let requires_point = match get_target_for_skill(name) {
        TargetType::None => false,
        TargetType::Tile => true,
        TargetType::Enemy => true,
    };

    if requires_point != target.is_some() {
        panic!("invoke_skill for {} called with wrong targeting param state.", name);
    }
}

pub fn invoke_skill(ecs: &mut World, name: &str, target: Option<&Point>) {
    assert_correct_targeting(name, target);
    ecs.log(&format!("Invoking {}", name));
}

#[cfg(test)]
mod tests {
    use super::super::create_world;
    use super::*;

    #[test]
    #[should_panic]
    fn panic_if_wrong_targeting() {
        let mut ecs = create_world();
        invoke_skill(&mut ecs, "TestNone", Some(&Point::init(2, 2)));
    }

    #[test]
    #[should_panic]
    fn panic_if_missing_targeting() {
        let mut ecs = create_world();
        invoke_skill(&mut ecs, "TestTile", None);
    }
}
