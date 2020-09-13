use serde::{Deserialize, Serialize};
use specs::prelude::*;

use rand::distributions::{Distribution, Standard};
use rand::prelude::*;

use super::*;
use crate::atlas::{Direction, EasyECS, EasyMutECS, SizedPoint};

#[macro_export]
macro_rules! try_behavior {
    ($x:expr) => {
        if $x {
            return;
        }
    };
}

#[macro_export]
macro_rules! try_behavior_and_if {
    ($x:expr, $y: expr) => {
        if $x {
            $y;
            return;
        }
    };
}

#[macro_export]
macro_rules! do_behavior {
    ($x:expr) => {
        $x;
        return;
    };
}

#[derive(Clone, Copy, Deserialize, Serialize, is_enum_variant)]
#[allow(dead_code)]
pub enum BehaviorKind {
    None,
    Bird,
    Egg,
    BirdAdd,
    Elementalist,
    WaterElemental,
    FireElemental,
    WindElemental,
    EarthElemental,
    TickDamage,
    Explode,
    Orb,
}

pub fn take_enemy_action(ecs: &mut World, enemy: &Entity) {
    let behavior = { ecs.read_storage::<BehaviorComponent>().grab(*enemy).behavior };
    match behavior {
        BehaviorKind::None => wait(ecs, *enemy),
        BehaviorKind::Bird => super::content::bird::bird_action(ecs, enemy),
        BehaviorKind::BirdAdd => super::content::bird::bird_add_action(ecs, enemy),
        BehaviorKind::Egg => super::content::bird::egg_action(ecs, enemy),
        BehaviorKind::Elementalist => super::content::elementalist::elementalist_action(ecs, enemy),
        BehaviorKind::WaterElemental => super::content::elementalist::water_elemental_action(ecs, enemy),
        BehaviorKind::FireElemental => super::content::elementalist::fire_elemental_action(ecs, enemy),
        BehaviorKind::WindElemental => super::content::elementalist::wind_elemental_action(ecs, enemy),
        BehaviorKind::EarthElemental => super::content::elementalist::earth_elemental_action(ecs, enemy),
        BehaviorKind::Explode => begin_explode(ecs, &enemy),
        BehaviorKind::TickDamage => {
            wait(ecs, *enemy);
            tick_damage(ecs, enemy);
            let should_die = {
                if let Some(d) = &mut ecs.write_storage::<DurationComponent>().get_mut(*enemy) {
                    d.duration -= 1;
                    d.duration == 0
                } else {
                    false
                }
            };
            if should_die {
                ecs.delete_entity(*enemy).unwrap();
            }
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
    let mut directions = vec![Direction::North, Direction::West, Direction::South, Direction::East];
    directions.shuffle(random);
    directions
}

pub fn coin_flip(ecs: &mut World) -> bool {
    let random = &mut ecs.fetch_mut::<RandomComponent>().rand;
    random.gen_bool(0.5)
}

pub fn get_random_full_direction_list(ecs: &mut World) -> Vec<Direction> {
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

pub fn use_skill_at_player_if_in_range(ecs: &mut World, enemy: &Entity, skill_name: &str) -> bool {
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

pub fn use_skill_with_random_target(ecs: &mut World, enemy: &Entity, skill_name: &str, range: u32) -> bool {
    let mut target = ecs.get_position(&find_player(ecs));

    let range = {
        let random = &mut ecs.fetch_mut::<RandomComponent>().rand;
        random.gen_range(0, range)
    };

    for _ in 0..range {
        let direction = get_random_direction_list(ecs)[0];
        if let Some(t) = direction.sized_point_in_direction(&target) {
            target = t;
        }
    }

    if is_good_target(ecs, enemy, &get_skill(skill_name), target.origin) {
        invoke_skill(ecs, enemy, skill_name, Some(target.origin));
        return true;
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
        move_character_action(ecs, *enemy, current_position.move_to(path[1]))
    } else {
        false
    }
}

pub fn use_no_target_skill_with_cooldown(ecs: &mut World, enemy: &Entity, skill_name: &str, cooldown: u32) -> bool {
    if check_behavior_cooldown(ecs, enemy, skill_name, cooldown) {
        if use_skill(ecs, enemy, skill_name) {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
pub fn use_random_target_skill_with_cooldown(ecs: &mut World, enemy: &Entity, skill_name: &str, cooldown: u32, range: u32) -> bool {
    if check_behavior_cooldown(ecs, enemy, skill_name, cooldown) {
        if use_skill_with_random_target(ecs, enemy, skill_name, range) {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
pub fn use_player_target_skill_with_cooldown(ecs: &mut World, enemy: &Entity, skill_name: &str, cooldown: u32) -> bool {
    if check_behavior_cooldown(ecs, enemy, skill_name, cooldown) {
        if use_skill_at_player_if_in_range(ecs, enemy, skill_name) {
            return true;
        }
    }
    false
}

pub fn flip_value(ecs: &World, enemy: &Entity, key: &str, left: u32, right: u32) -> u32 {
    if has_behavior_value(ecs, enemy, key) {
        clear_behavior_value(ecs, enemy, key);
        right
    } else {
        set_behavior_value(ecs, enemy, key, 1);
        left
    }
}

pub fn has_behavior_value(ecs: &World, enemy: &Entity, key: &str) -> bool {
    ecs.read_storage::<BehaviorComponent>().grab(*enemy).info.contains_key(key)
}

pub fn clear_behavior_value(ecs: &World, enemy: &Entity, key: &str) {
    ecs.write_storage::<BehaviorComponent>().grab_mut(*enemy).info.remove(key);
}

#[allow(dead_code)]
pub fn get_behavior_value(ecs: &World, enemy: &Entity, key: &str, default: u32) -> u32 {
    *ecs.read_storage::<BehaviorComponent>().grab(*enemy).info.get(key).unwrap_or(&default)
}

pub fn get_behavior_value_calculate(ecs: &World, enemy: &Entity, key: &str, default: &impl Fn(&World) -> u32) -> u32 {
    // Must copy value so we don't hold lock on read_storage when calling closure
    // which will likely rewire write_storage to flip a bit
    let value = { ecs.read_storage::<BehaviorComponent>().grab(*enemy).info.get(key).copied() };
    value.unwrap_or_else(|| default(ecs))
}

pub fn set_behavior_value(ecs: &World, enemy: &Entity, key: &str, value: u32) {
    ecs.write_storage::<BehaviorComponent>().grab_mut(*enemy).info.insert(key.to_string(), value);
}

pub fn check_behavior_cooldown(ecs: &World, enemy: &Entity, key: &str, length: u32) -> bool {
    check_behavior_cooldown_calculate(ecs, enemy, key, |_| length)
}

pub fn check_behavior_cooldown_calculate(ecs: &World, enemy: &Entity, key: &str, length: impl Fn(&World) -> u32) -> bool {
    let value = get_behavior_value_calculate(ecs, enemy, key, &length);
    if value <= 1 {
        set_behavior_value(ecs, enemy, key, length(ecs));
        true
    } else {
        set_behavior_value(ecs, enemy, key, value - 1);
        false
    }
}

#[allow(dead_code)]
pub fn check_behavior_ammo(ecs: &World, enemy: &Entity, key: &str, ammo: u32) -> bool {
    check_behavior_ammo_calculate(ecs, enemy, key, |_| ammo)
}

pub fn check_behavior_ammo_calculate(ecs: &World, enemy: &Entity, key: &str, ammo: impl Fn(&World) -> u32) -> bool {
    let value = get_behavior_value_calculate(ecs, enemy, key, &ammo);
    if value >= 1 {
        set_behavior_value(ecs, enemy, key, value - 1);
        true
    } else {
        set_behavior_value(ecs, enemy, key, ammo(ecs));
        false
    }
}

pub fn check_behavior_single_use_ammo(ecs: &World, enemy: &Entity, key: &str, ammo: u32) -> bool {
    let value = get_behavior_value(ecs, enemy, key, ammo);
    if value >= 1 {
        set_behavior_value(ecs, enemy, key, value - 1);
        true
    } else {
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

    #[test]
    fn behavior_value_flip() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_map().build();
        let target = find_at(&ecs, 2, 2);
        ecs.shovel(target, BehaviorComponent::init(BehaviorKind::None));

        assert_eq!(2, flip_value(&ecs, &target, "TestKey", 2, 3));
        assert_eq!(3, flip_value(&ecs, &target, "TestKey", 2, 3));
        assert_eq!(2, flip_value(&ecs, &target, "TestKey", 2, 3));
        assert_eq!(3, flip_value(&ecs, &target, "TestKey", 2, 3));
    }

    #[test]
    fn behavior_value_set_clear() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_map().build();
        let target = find_at(&ecs, 2, 2);
        ecs.shovel(target, BehaviorComponent::init(BehaviorKind::None));

        assert!(!has_behavior_value(&ecs, &target, "TestKey"));
        set_behavior_value(&ecs, &target, "TestKey", 1);
        assert!(has_behavior_value(&ecs, &target, "TestKey"));
        clear_behavior_value(&ecs, &target, "TestKey");
        assert!(!has_behavior_value(&ecs, &target, "TestKey"));
    }

    #[test]
    fn behavior_ammo_value() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_map().build();
        let target = find_at(&ecs, 2, 2);
        ecs.shovel(target, BehaviorComponent::init(BehaviorKind::None));

        assert!(check_behavior_ammo(&ecs, &target, "TestKey", 3));
        assert!(check_behavior_ammo(&ecs, &target, "TestKey", 3));
        assert!(check_behavior_ammo(&ecs, &target, "TestKey", 3));
        assert!(!check_behavior_ammo(&ecs, &target, "TestKey", 3));

        assert!(check_behavior_ammo_calculate(&ecs, &target, "TestKey", |_| 3));
        assert!(check_behavior_ammo_calculate(&ecs, &target, "TestKey", |_| 3));
        assert!(check_behavior_ammo_calculate(&ecs, &target, "TestKey", |_| 3));
        assert!(!check_behavior_ammo_calculate(&ecs, &target, "TestKey", |_| 3));
    }

    #[test]
    fn single_use_ammo() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_map().build();
        let target = find_at(&ecs, 2, 2);
        ecs.shovel(target, BehaviorComponent::init(BehaviorKind::None));

        assert!(check_behavior_single_use_ammo(&ecs, &target, "TestKey", 3));
        assert!(check_behavior_single_use_ammo(&ecs, &target, "TestKey", 3));
        assert!(check_behavior_single_use_ammo(&ecs, &target, "TestKey", 3));
        assert!(!check_behavior_single_use_ammo(&ecs, &target, "TestKey", 3));
        assert!(!check_behavior_single_use_ammo(&ecs, &target, "TestKey", 3));
        assert!(!check_behavior_single_use_ammo(&ecs, &target, "TestKey", 3));
    }

    #[test]
    fn move_towards_player_changes_location() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 4, 0).with_map().build();
        let target = find_at(&ecs, 2, 4);
        move_towards_player(&mut ecs, &target);
        wait_for_animations(&mut ecs);
        assert_character_at(&ecs, 2, 3);
    }

    #[test]
    fn ticks_damage() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_map().build();
        let player = find_at(&ecs, 2, 2);
        let damage_source = make_test_character(&mut ecs, SizedPoint::init(2, 2), 0);
        ecs.shovel(damage_source, DurationComponent::init(2));
        ecs.shovel(damage_source, BehaviorComponent::init(BehaviorKind::TickDamage));
        ecs.shovel(
            damage_source,
            AttackComponent::init(Point::init(2, 2), Damage::init(1), AttackKind::DamageTick, Some(Point::init(2, 2))),
        );
        ecs.write_storage::<CharacterInfoComponent>().remove(damage_source);

        for _ in 0..2 {
            let target_health = ecs.get_defenses(&player).health;
            take_enemy_action(&mut ecs, &damage_source);
            add_ticks(&mut ecs, 100);
            assert_ne!(ecs.get_defenses(&player).health, target_health);
        }

        assert_eq!(1, find_all_entities(&ecs).len());
    }
}
