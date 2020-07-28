use specs::prelude::*;

use super::*;
use crate::atlas::{Point, SizedPoint};

pub fn begin_move(ecs: &World, entity: &Entity, new_position: SizedPoint) {
    let frame = ecs.read_resource::<FrameComponent>();
    let mut animations = ecs.write_storage::<AnimationComponent>();
    let position = ecs.get_position(entity);

    let animation = AnimationComponent::movement(position.origin, new_position.origin, frame.current_frame, frame.current_frame + 8);
    animations.insert(*entity, animation).unwrap();
}

pub fn complete_move(ecs: &World, entity: &Entity, new_position: &Point) {
    let mut positions = ecs.write_storage::<PositionComponent>();
    let position = &mut positions.get_mut(*entity).unwrap();
    position.move_to(*new_position);
}

pub fn point_in_direction(initial: &SizedPoint, direction: Direction) -> Option<SizedPoint> {
    let is_valid = initial.all_positions().iter().all(|&p| match direction {
        Direction::North => p.y > 0,
        Direction::South => p.y < MAX_MAP_TILES - 1,
        Direction::East => p.x < MAX_MAP_TILES - 1,
        Direction::West => p.x > 0,
    });

    if is_valid {
        match direction {
            Direction::North => Some(SizedPoint::init_multi(initial.origin.x, initial.origin.y - 1, initial.width, initial.height)),
            Direction::South => Some(SizedPoint::init_multi(initial.origin.x, initial.origin.y + 1, initial.width, initial.height)),
            Direction::East => Some(SizedPoint::init_multi(initial.origin.x + 1, initial.origin.y, initial.width, initial.height)),
            Direction::West => Some(SizedPoint::init_multi(initial.origin.x - 1, initial.origin.y, initial.width, initial.height)),
        }
    } else {
        None
    }
}

// Is an area clear of all elements with PositionComponent and CharacterInfoComponent _except_ the invoker
pub fn is_area_clear(ecs: &World, area: &[Point], invoker: &Entity) -> bool {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let positions = ecs.read_storage::<PositionComponent>();
    let char_info = ecs.read_storage::<CharacterInfoComponent>();
    let map = &ecs.read_resource::<MapComponent>().map;

    for (entity, position, _) in (&entities, &positions, &char_info).join() {
        for p in area.iter() {
            if !map.is_in_bounds(&p) || !map.is_walkable(&p) {
                return false;
            }
            if *invoker != entity && position.position.contains_point(&p) {
                return false;
            }
        }
    }
    true
}

pub fn can_move_character(ecs: &World, mover: &Entity, new: SizedPoint) -> bool {
    is_area_clear(ecs, &new.all_positions(), mover)
}

pub fn move_character(ecs: &mut World, entity: Entity, new: SizedPoint) -> bool {
    if !can_move_character(ecs, &entity, new) {
        return false;
    }

    begin_move(ecs, &entity, new);
    spend_time(ecs, &entity, MOVE_ACTION_COST);
    true
}

pub fn wait(ecs: &mut World, entity: Entity) {
    spend_time(ecs, &entity, BASE_ACTION_COST);
}

#[cfg(test)]
pub fn wait_for_animations(ecs: &mut World) {
    loop {
        ecs.write_resource::<FrameComponent>().current_frame += 1;
        tick_animations(ecs, ecs.read_resource::<FrameComponent>().current_frame).unwrap();

        let animations = ecs.read_storage::<AnimationComponent>();
        if (animations).join().count() == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::create_world;
    use super::*;
    use crate::atlas::SizedPoint;

    fn add_character(ecs: &mut World, position: SizedPoint) -> Entity {
        ecs.create_entity()
            .with(PositionComponent::init(position))
            .with(CharacterInfoComponent::init(Character::init()))
            .with(TimeComponent::init(100))
            .build()
    }

    fn assert_position(ecs: &World, entity: &Entity, expected: Point) {
        let position = ecs.get_position(entity);
        assert_eq!(position.single_position().x, expected.x);
        assert_eq!(position.single_position().y, expected.y);
    }

    #[test]
    fn point_off_map() {
        assert_eq!(None, point_in_direction(&SizedPoint::init(5, 0), Direction::North));
        assert_eq!(None, point_in_direction(&SizedPoint::init(5, MAX_MAP_TILES - 1), Direction::South));
        assert_eq!(None, point_in_direction(&SizedPoint::init(0, 5), Direction::West));
        assert_eq!(None, point_in_direction(&SizedPoint::init(MAX_MAP_TILES - 1, 5), Direction::East));
    }

    #[test]
    fn walk_into_clear() {
        let mut ecs = create_world();
        ecs.insert(MapComponent::init(Map::init_empty()));
        let entity = add_character(&mut ecs, SizedPoint::init(2, 2));
        assert_position(&ecs, &entity, Point::init(2, 2));

        let success = move_character(&mut ecs, entity, SizedPoint::init(2, 3));
        assert_eq!(true, success);
        wait_for_animations(&mut ecs);

        assert_position(&ecs, &entity, Point::init(2, 3));
        assert_eq!(0, ecs.read_storage::<TimeComponent>().get(entity).unwrap().ticks);
    }

    #[test]
    fn walk_into_non_characters() {
        let mut ecs = create_world();
        ecs.insert(MapComponent::init(Map::init_empty()));
        let entity = add_character(&mut ecs, SizedPoint::init(2, 2));
        ecs.create_entity()
            .with(PositionComponent::init(SizedPoint::init(2, 3)))
            .with(FieldComponent::init(255, 0, 0))
            .build();

        assert_position(&ecs, &entity, Point::init(2, 2));

        let success = move_character(&mut ecs, entity, SizedPoint::init(2, 3));
        assert_eq!(true, success);
        wait_for_animations(&mut ecs);

        assert_position(&ecs, &entity, Point::init(2, 3));
    }

    #[test]
    fn unable_to_walk_into_unwalkable() {
        let mut ecs = create_world();
        let mut map = Map::init_empty();
        map.set_walkable(&Point::init(2, 3), false);
        ecs.insert(MapComponent::init(map));
        let entity = add_character(&mut ecs, SizedPoint::init(2, 2));
        assert_position(&ecs, &entity, Point::init(2, 2));

        let success = move_character(&mut ecs, entity, SizedPoint::init(2, 3));
        assert_eq!(false, success);
        wait_for_animations(&mut ecs);

        assert_position(&ecs, &entity, Point::init(2, 2))
    }

    #[test]
    fn unable_to_walk_into_another() {
        let mut ecs = create_world();
        ecs.insert(MapComponent::init(Map::init_empty()));
        let entity = add_character(&mut ecs, SizedPoint::init(2, 2));
        ecs.create_entity()
            .with(PositionComponent::init(SizedPoint::init(2, 3)))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();

        assert_position(&ecs, &entity, Point::init(2, 2));

        let success = move_character(&mut ecs, entity, SizedPoint::init(2, 3));
        assert_eq!(false, success);
        wait_for_animations(&mut ecs);

        assert_position(&ecs, &entity, Point::init(2, 2));
    }

    #[test]
    fn walk_off_map() {
        let mut ecs = create_world();
        ecs.insert(MapComponent::init(Map::init_empty()));
        let entity = add_character(&mut ecs, SizedPoint::init(13, 13));
        assert_position(&ecs, &entity, Point::init(13, 13));

        let success = move_character(&mut ecs, entity, SizedPoint::init(13, 14));
        assert_eq!(false, success);
        wait_for_animations(&mut ecs);

        assert_position(&ecs, &entity, Point::init(13, 13));
    }

    #[test]
    fn multi_walks_into_single() {
        let mut ecs = create_world();
        ecs.insert(MapComponent::init(Map::init_empty()));
        add_character(&mut ecs, SizedPoint::init(2, 2));
        let bottom = add_character(&mut ecs, SizedPoint::init_multi(2, 4, 2, 2));

        let success = move_character(&mut ecs, bottom, SizedPoint::init_multi(2, 3, 2, 2));
        assert_eq!(false, success);
        wait_for_animations(&mut ecs);
    }
}
