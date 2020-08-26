use serde::{Deserialize, Serialize};
use specs::prelude::*;

use rand::distributions::{Distribution, Standard};
use rand::prelude::*;

use super::*;
use crate::atlas::{EasyECS, SizedPoint};

#[derive(Clone, Copy, Deserialize, Serialize)]
#[allow(dead_code)]
pub enum BehaviorKind {
    None,
    Random,
    Explode,
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

pub fn take_enemy_action(ecs: &mut World, enemy: &Entity) {
    let behavior = { ecs.read_storage::<BehaviorComponent>().grab(*enemy).behavior };
    match behavior {
        BehaviorKind::None => wait(ecs, *enemy),
        BehaviorKind::Random => {
            let position = ecs.get_position(enemy);
            if let Some(direction) = get_random_direction(ecs, position, enemy) {
                let point = point_in_direction(&position, direction).unwrap();
                move_character_action(ecs, *enemy, point);
            }
        }
        BehaviorKind::Explode => {
            begin_explode(ecs, &enemy);
        }
    };
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
    fn random_behavior() {
        let mut ecs = create_test_state().with_character(2, 2, 100).with_map().build();
        let character = find_at(&ecs, 2, 2);
        ecs.shovel(character, BehaviorComponent::init(BehaviorKind::Random));

        take_enemy_action(&mut ecs, &character);
        wait_for_animations(&mut ecs);

        let final_position = ecs.get_position(&character);
        assert_ne!(final_position.origin, Point::init(2, 2));
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
}
