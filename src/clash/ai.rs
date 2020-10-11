use std::cmp;

use serde::{Deserialize, Serialize};
use specs::prelude::*;

use rand::distributions::{Distribution, Standard};
use rand::prelude::*;

use super::*;
use crate::atlas::prelude::*;

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
    SimpleGolem,
    TickDamage,
    Explode,
    Orb,
}

pub fn take_enemy_action(ecs: &mut World, enemy: Entity) {
    let behavior = { ecs.read_storage::<BehaviorComponent>().grab(enemy).behavior };
    match behavior {
        BehaviorKind::None => wait(ecs, enemy),
        BehaviorKind::Bird => super::content::bird::bird_action(ecs, enemy),
        BehaviorKind::BirdAdd => super::content::bird::bird_add_action(ecs, enemy),
        BehaviorKind::Egg => super::content::bird::egg_action(ecs, enemy),
        BehaviorKind::Elementalist => super::content::elementalist::elementalist_action(ecs, enemy),
        BehaviorKind::WaterElemental => super::content::elementalist::water_elemental_action(ecs, enemy),
        BehaviorKind::FireElemental => super::content::elementalist::fire_elemental_action(ecs, enemy),
        BehaviorKind::WindElemental => super::content::elementalist::wind_elemental_action(ecs, enemy),
        BehaviorKind::EarthElemental => super::content::elementalist::earth_elemental_action(ecs, enemy),
        BehaviorKind::SimpleGolem => super::content::tutorial::golem_action(ecs, enemy),
        BehaviorKind::Explode => begin_explode(ecs, enemy),
        BehaviorKind::TickDamage => {
            wait(ecs, enemy);
            tick_damage(ecs, enemy);
            let should_die = {
                if let Some(d) = &mut ecs.write_storage::<DurationComponent>().get_mut(enemy) {
                    d.duration -= 1;
                    d.duration == 0
                } else {
                    false
                }
            };
            if should_die {
                ecs.delete_entity(enemy).unwrap();
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

pub fn is_tile_safe(ecs: &World, position: &SizedPoint) -> bool {
    find_field_at_location(ecs, position).is_none()
}

fn get_random_direction_to_move(ecs: &mut World, position: SizedPoint, enemy: Entity) -> Option<Direction> {
    let random = &mut ecs.fetch_mut::<RandomComponent>().rand;
    for _ in 0..10 {
        let direction: Direction = random.gen();
        if let Some(point) = point_in_direction(&position, direction) {
            if can_move_character(ecs, enemy, point) && is_tile_safe(ecs, &point) {
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

pub fn move_randomly(ecs: &mut World, enemy: Entity) -> bool {
    let position = ecs.get_position(enemy);
    if let Some(direction) = get_random_direction_to_move(ecs, position, enemy) {
        let point = point_in_direction(&position, direction).unwrap();
        move_character_action(ecs, enemy, point)
    } else {
        false
    }
}

pub fn move_towards_player(ecs: &mut World, enemy: Entity) -> bool {
    let current_position = ecs.get_position(enemy);
    let player_position = ecs.get_position(find_player(ecs));
    if let Some(path) = current_position.line_to(player_position.origin) {
        let next = current_position.move_to(path[1]);
        if is_tile_safe(ecs, &next) {
            return move_character_action(ecs, enemy, next);
        }
    }
    false
}

pub fn use_skill(ecs: &mut World, enemy: Entity, skill_name: &str) -> bool {
    use_skill_core(ecs, enemy, skill_name, None)
}

pub fn use_skill_at_position(ecs: &mut World, enemy: Entity, skill_name: &str, target_point: Point) -> bool {
    use_skill_core(ecs, enemy, skill_name, Some(target_point))
}

fn use_skill_core(ecs: &mut World, enemy: Entity, skill_name: &str, target_point: Option<Point>) -> bool {
    if can_invoke_skill(ecs, enemy, get_skill(skill_name), target_point) {
        invoke_skill(ecs, enemy, skill_name, target_point);
        return true;
    }
    false
}

pub fn use_skill_at_player_if_in_range(ecs: &mut World, enemy: Entity, skill_name: &str) -> bool {
    let current_position = ecs.get_position(enemy);
    let player_position = ecs.get_position(find_player(ecs));
    if let Some((_, target_point, distance)) = current_position.distance_to_multi_with_endpoints(player_position) {
        let skill = get_skill(skill_name);
        if distance <= skill.range.unwrap() {
            if can_invoke_skill(ecs, enemy, skill, Some(target_point)) {
                invoke_skill(ecs, enemy, skill_name, Some(target_point));
                return true;
            }
        }
    }
    false
}

pub fn use_skill_with_random_target(ecs: &mut World, enemy: Entity, skill_name: &str, range: u32) -> bool {
    let skill = get_skill(skill_name);
    // Early return for lack of resources before trying many target squares
    if !has_resources_for_skill(ecs, enemy, &skill) {
        return false;
    }

    let mut target = ecs.get_position(find_player(ecs));

    let mut range = {
        let random = &mut ecs.fetch_mut::<RandomComponent>().rand;
        random.gen_range(0, range)
    };

    // Try 20 times for a valid target
    for attempt in 0..20 {
        for _ in 0..range {
            let direction = get_random_direction_list(ecs)[0];
            if let Some(t) = direction.sized_point_in_direction(&target) {
                target = t;
            }
        }

        if can_invoke_skill(ecs, enemy, &skill, Some(target.origin)) {
            invoke_skill(ecs, enemy, skill_name, Some(target.origin));
            return true;
        }

        // Every 8 reduce range by 1 to search closer (less in water)
        if attempt % 8 == 7 {
            range = cmp::max(range as i32 - 1, 1) as u32;
        }
    }
    false
}

pub fn distance_to_player(ecs: &mut World, enemy: Entity) -> Option<u32> {
    let current_position = ecs.get_position(enemy);
    let player_position = ecs.get_position(find_player(ecs));
    current_position.distance_to_multi(player_position)
}

pub fn check_for_cone_striking_player(ecs: &World, enemy: Entity, size: u32) -> Option<Point> {
    let position = ecs.get_position(enemy);
    let player_position = ecs.get_position(find_player(&ecs));
    for origin in position.all_positions() {
        for d in &[Direction::North, Direction::East, Direction::South, Direction::East] {
            if origin.get_cone(*d, size).iter().any(|p| player_position.contains_point(p)) {
                return Some(d.point_in_direction(&origin).unwrap());
            }
        }
    }
    None
}

pub fn any_ally_without_buff_in_range(ecs: &World, enemy: Entity, buff: StatusKind, range: u32) -> Option<Entity> {
    let position = ecs.get_position(enemy);
    let player = find_player(ecs);
    find_all_characters(ecs)
        .iter()
        .filter(|&&c| c != player)
        .filter(|&&c| !ecs.has_status(c, buff))
        .find(|&&c| position.distance_to_multi(ecs.get_position(c)).unwrap_or(std::u32::MAX) <= range)
        .copied()
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn no_behavior() {
        let mut ecs = create_test_state().with_character(2, 2, 100).build();
        let character = find_at(&ecs, 2, 2);
        ecs.shovel(character, BehaviorComponent::init(BehaviorKind::None));

        take_enemy_action(&mut ecs, character);
        wait_for_animations(&mut ecs);

        let final_position = ecs.get_position(character);
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
                AttackKind::Explode(ExplosionKind::Bomb, 2),
                Some(Point::init(2, 2)),
            ))
            .build();

        let starting_health = ecs.get_defenses(target).health;
        take_enemy_action(&mut ecs, character);
        wait_for_animations(&mut ecs);

        assert_eq!(0, ecs.read_storage::<BehaviorComponent>().count());
        assert!(ecs.get_defenses(target).health < starting_health);
    }

    #[test]
    fn move_towards_player_changes_location() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 4, 0).with_map().build();
        let target = find_at(&ecs, 2, 4);
        move_towards_player(&mut ecs, target);
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
            let target_health = ecs.get_defenses(player).health;
            take_enemy_action(&mut ecs, damage_source);
            add_ticks(&mut ecs, 100);
            assert_ne!(ecs.get_defenses(player).health, target_health);
        }

        assert_eq!(1, find_all_entities(&ecs).len());
    }

    #[test]
    fn move_random_avoids_field() {
        let mut ecs = create_test_state().with_player(1, 1, 0).with_character(2, 4, 0).with_map().build();
        let player = find_player(&ecs);

        // Only 1,4 and 2,4 are safe points
        for p in &[Point::init(2, 3), Point::init(3, 4), Point::init(2, 5)] {
            invoke_skill(&mut ecs, player, "TestField", Some(*p));
            wait_for_animations(&mut ecs);
        }

        let target = find_at(&ecs, 2, 4);
        move_randomly(&mut ecs, target);
        wait_for_animations(&mut ecs);

        assert!(find_character_at_location(&ecs, Point::init(2, 4)).is_some() || find_character_at_location(&ecs, Point::init(1, 4)).is_some());
    }

    #[test]
    fn move_towards_player_avoid_field() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(2, 4, 0).with_map().build();
        let player = find_player(&ecs);
        invoke_skill(&mut ecs, player, "TestField", Some(Point::init(2, 3)));
        wait_for_animations(&mut ecs);

        let target = find_at(&ecs, 2, 4);
        move_towards_player(&mut ecs, target);
        wait_for_animations(&mut ecs);
        assert_character_at(&ecs, 2, 4);
    }
}
