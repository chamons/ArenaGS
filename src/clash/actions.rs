use specs::prelude::*;

use super::{invoke_skill, move_character, PlayerComponent, Point, PositionComponent, MAX_MAP_TILES};

pub enum Direction {
    North,
    South,
    East,
    West,
}

fn find_player(ecs: &World) -> Option<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let players = ecs.read_storage::<PlayerComponent>();

    if let Some((entity, _)) = (&entities, &players).join().next() {
        return Some(entity);
    }
    None
}

fn point_in_direction(initial: &Point, direction: Direction) -> Option<Point> {
    let is_valid = match direction {
        Direction::North => initial.y > 0,
        Direction::South => initial.y < MAX_MAP_TILES - 1,
        Direction::East => initial.x < MAX_MAP_TILES - 1,
        Direction::West => initial.x > 0,
    };

    if is_valid {
        match direction {
            Direction::North => Some(Point::init(initial.x, initial.y - 1)),
            Direction::South => Some(Point::init(initial.x, initial.y + 1)),
            Direction::East => Some(Point::init(initial.x + 1, initial.y)),
            Direction::West => Some(Point::init(initial.x - 1, initial.y)),
        }
    } else {
        None
    }
}

pub fn player_move(ecs: &mut World, direction: Direction) {
    let player = find_player(ecs).unwrap();
    let new_position = {
        let positions = ecs.read_storage::<PositionComponent>();
        let position_component = positions.get(player).unwrap();
        point_in_direction(&position_component.single_position(), direction)
    };

    if let Some(new_position) = new_position {
        move_character(ecs, player, new_position);
    }
}

pub fn player_use_skill(ecs: &mut World, name: &str, target: Option<Point>) {
    invoke_skill(ecs, name, None);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_off_map() {
        assert_eq!(None, point_in_direction(&Point::init(5, 0), Direction::North));
        assert_eq!(None, point_in_direction(&Point::init(5, MAX_MAP_TILES - 1), Direction::South));
        assert_eq!(None, point_in_direction(&Point::init(0, 5), Direction::West));
        assert_eq!(None, point_in_direction(&Point::init(MAX_MAP_TILES - 1, 5), Direction::East));
    }

    #[test]
    fn move_not_current_actor() {}

    #[test]
    fn move_spends_time() {}

    #[test]
    fn use_skill_not_current_actor() {}

    #[test]
    fn use_skill_spends_time() {}
}
