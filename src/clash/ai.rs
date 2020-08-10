use specs::prelude::*;
use specs_derive::Component;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use super::*;
use crate::atlas::EasyECS;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum BehaviorKind {
    None,
    Random,
    Explode,
}

#[derive(Component)]
pub struct BehaviorComponent {
    behavior: BehaviorKind,
}

impl BehaviorComponent {
    pub fn init(behavior: BehaviorKind) -> BehaviorComponent {
        BehaviorComponent { behavior }
    }
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

pub fn take_enemy_action(ecs: &mut World, enemy: &Entity) {
    let behavior = { ecs.read_storage::<BehaviorComponent>().grab(*enemy).behavior };
    match behavior {
        BehaviorKind::None => wait(ecs, *enemy),
        BehaviorKind::Random => {
            let position = ecs.get_position(enemy);
            for _ in 0..5 {
                let direction: Direction = rand::random();
                if let Some(point) = point_in_direction(&position, direction) {
                    if can_move_character(ecs, enemy, point) {
                        let did_move = move_character(ecs, *enemy, point);
                        if did_move {
                            return;
                        }
                    }
                }
            }
        }
        BehaviorKind::Explode => {
            explode(ecs, &enemy);
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
        let character = ecs
            .create_entity()
            .with(BehaviorComponent::init(BehaviorKind::Explode))
            .with(PositionComponent::init(SizedPoint::init(2, 2)))
            .with(TimeComponent::init(100))
            .with(AttackComponent::init(Point::init(2, 2), 2, AttackKind::Explode(2)))
            .build();

        take_enemy_action(&mut ecs, &character);
        wait_for_animations(&mut ecs);

        assert_eq!(0, ecs.read_storage::<BehaviorComponent>().count());
        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }
}
