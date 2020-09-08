use serde::{Deserialize, Serialize};
use specs::prelude::*;

use rand::distributions::{Distribution, Standard};
use rand::prelude::*;

use super::*;
use crate::atlas::{EasyECS, EasyMutECS, SizedPoint};

#[macro_export]
macro_rules! try_behavior {
    ($x:expr) => {
        if $x {
            return;
        }
    };
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[allow(dead_code)]
pub enum BehaviorKind {
    None,
    Bird,
    Explode,
    Orb,
}

pub fn take_enemy_action(ecs: &mut World, enemy: &Entity) {
    let behavior = { ecs.read_storage::<BehaviorComponent>().grab(*enemy).behavior };
    match behavior {
        BehaviorKind::None => wait(ecs, *enemy),
        BehaviorKind::Bird => super::content::bird::take_action(ecs, enemy),
        BehaviorKind::Explode => {
            begin_explode(ecs, &enemy);
        }
        BehaviorKind::Orb => move_orb(ecs, enemy),
    };
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            3 => Direction::West,
            _ => unreachable!(),
        }
    }
}

fn get_random_direction(ecs: &mut World, position: SizedPoint, enemy: &Entity) -> Option<Direction> {
    let random = &mut ecs.fetch_mut::<RandomComponent>().rand;
    for _ in 0..5 {
        let direction: Direction = random.gen();
        if let Some(point) = point_in_direction(&position, direction) {
            if can_move_character(ecs, enemy, point) {
                return Some(direction);
            }
        }
    }
    None
}

pub fn get_random_direction_list(ecs: &mut World) -> Vec<Direction> {
    let random = &mut ecs.fetch_mut::<RandomComponent>().rand;
    let mut directions = vec![
        Direction::North,
        Direction::NorthWest,
        Direction::West,
        Direction::SouthWest,
        Direction::South,
        Direction::SouthEast,
        Direction::East,
        Direction::NorthEast,
    ];
    directions.shuffle(random);
    directions
}

pub fn move_randomly(ecs: &mut World, enemy: &Entity) -> bool {
    let position = ecs.get_position(enemy);
    if let Some(direction) = get_random_direction(ecs, position, enemy) {
        let point = point_in_direction(&position, direction).unwrap();
        move_character_action(ecs, *enemy, point)
    } else {
        false
    }
}

#[allow(dead_code)]
pub fn use_skill(ecs: &mut World, enemy: &Entity, skill_name: &str) -> bool {
    if can_invoke_skill(ecs, enemy, get_skill(skill_name), None) {
        invoke_skill(ecs, enemy, skill_name, None);
        true
    } else {
        false
    }
}

pub fn use_skill_if_in_range(ecs: &mut World, enemy: &Entity, skill_name: &str) -> bool {
    let current_position = ecs.get_position(enemy);
    let player_position = ecs.get_position(&find_player(ecs));
    if let Some((_, target_point, distance)) = current_position.distance_to_multi_with_endpoints(player_position) {
        let skill = get_skill(skill_name);
        if distance <= skill.range.unwrap() {
            if is_good_target(ecs, enemy, skill, target_point) {
                invoke_skill(ecs, enemy, skill_name, Some(target_point));
                return true;
            }
        }
    }
    false
}

pub fn distance_to_player(ecs: &mut World, enemy: &Entity) -> Option<u32> {
    let current_position = ecs.get_position(enemy);
    let player_position = ecs.get_position(&find_player(ecs));
    current_position.distance_to_multi(player_position)
}

pub fn move_towards_player(ecs: &mut World, enemy: &Entity) -> bool {
    let current_position = ecs.get_position(enemy);
    let player_position = ecs.get_position(&find_player(ecs));
    if let Some(path) = current_position.line_to(player_position.origin) {
        move_character_action(ecs, *enemy, current_position.move_to(path[0]))
    } else {
        false
    }
}

pub fn use_skill_with_cooldown(ecs: &mut World, enemy: &Entity, skill_name: &str, cooldown: u32) -> bool {
    if check_behavior_cooldown(ecs, enemy, skill_name, cooldown) {
        if use_skill(ecs, enemy, skill_name) {
            return true;
        }
    }
    false
}

pub fn get_behavior_value(ecs: &World, enemy: &Entity, key: &str, default: u32) -> u32 {
    *ecs.read_storage::<BehaviorComponent>().grab(*enemy).info.get(key).unwrap_or(&default)
}

pub fn set_behavior_value(ecs: &World, enemy: &Entity, key: &str, value: u32) {
    ecs.write_storage::<BehaviorComponent>().grab_mut(*enemy).info.insert(key.to_string(), value);
}

pub fn check_behavior_cooldown(ecs: &World, enemy: &Entity, key: &str, length: u32) -> bool {
    let value = get_behavior_value(ecs, enemy, key, length);
    if value <= 1 {
        set_behavior_value(ecs, enemy, key, length);
        true
    } else {
        set_behavior_value(ecs, enemy, key, value - 1);
        false
    }
}
#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::atlas::{EasyMutWorld, Point, SizedPoint};

    #[test]
    fn no_behavior() {
        let mut ecs = create_test_state().with_character(2, 2, 100).build();
        let character = find_at(&ecs, 2, 2);
        ecs.shovel(character, BehaviorComponent::init(BehaviorKind::None));

        take_enemy_action(&mut ecs, &character);
        wait_for_animations(&mut ecs);

        let final_position = ecs.get_position(&character);
        assert_eq!(final_position.origin, Point::init(2, 2));
    }

    #[test]
    fn explode_behavior() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_map().build();
        let target = find_at(&ecs, 2, 2);
        let character = ecs
            .create_entity()
            .with(BehaviorComponent::init(BehaviorKind::Explode))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(TimeComponent::init(100))
            .with(AttackComponent::init(
                Point::init(2, 2),
                Damage::init(2),
                AttackKind::Explode(2),
                Some(Point::init(2, 2)),
            ))
            .build();

        let starting_health = ecs.get_defenses(&target).health;
        take_enemy_action(&mut ecs, &character);
        wait_for_animations(&mut ecs);

        assert_eq!(0, ecs.read_storage::<BehaviorComponent>().count());
        assert!(ecs.get_defenses(&target).health < starting_health);
    }

    #[test]
    fn behavior_value_checks() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_map().build();
        let target = find_at(&ecs, 2, 2);
        ecs.shovel(target, BehaviorComponent::init(BehaviorKind::None));

        assert!(!check_behavior_cooldown(&ecs, &target, "TestKey", 5));
        assert_eq!(4, get_behavior_value(&ecs, &target, "TestKey", 5));
        assert!(!check_behavior_cooldown(&ecs, &target, "TestKey", 5));
        assert!(!check_behavior_cooldown(&ecs, &target, "TestKey", 5));
        assert!(!check_behavior_cooldown(&ecs, &target, "TestKey", 5));
        assert!(check_behavior_cooldown(&ecs, &target, "TestKey", 5));
        assert_eq!(5, get_behavior_value(&ecs, &target, "TestKey", 5));
    }
}
