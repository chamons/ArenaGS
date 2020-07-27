use specs::prelude::*;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use super::{can_move_character, move_character, point_in_direction, wait, Direction, PositionComponent};

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
    let position = ecs.read_storage::<PositionComponent>().get(*enemy).unwrap().position;
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

    wait(ecs, *enemy);
}
