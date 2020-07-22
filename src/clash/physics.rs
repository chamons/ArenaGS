use specs::prelude::*;

use super::*;

pub fn move_character(ecs: &mut World, entity: Entity, new: Point) -> bool {
    let map = &ecs.read_resource::<MapComponent>().map;
    let positions = ecs.read_storage::<PositionComponent>();
    let char_info = ecs.read_storage::<CharacterInfoComponent>();

    if !map.is_in_bounds(&new) || !map.is_walkable(&new) {
        return false;
    }
    for (positions, _) in (&positions, &char_info).join() {
        if positions.contains_point(&new) {
            return false;
        }
    }
    true
}

pub fn move_character(ecs: &mut World, entity: Entity, new: Point) -> bool {
    if !can_move_character(ecs, new) {
        return false;
    }

    apply_move(ecs, &entity, &new);
    true
}

#[cfg(test)]
mod tests {
    use super::super::create_world;
    use super::*;

    fn add_character(ecs: &mut World, position: Point) -> Entity {
        ecs.create_entity()
            .with(PositionComponent::init(position.x, position.y))
            .with(CharacterInfoComponent::init(Character::init()))
            .build()
    }

    fn assert_position(ecs: &World, entity: &Entity, expected: Point) {
        let positions = ecs.read_storage::<PositionComponent>();
        let position = &positions.get(*entity).unwrap();
        assert_eq!(position.x(), expected.x);
        assert_eq!(position.y(), expected.y);
    }

    #[test]
    fn walk_into_clear() {
        let mut ecs = create_world();
        ecs.insert(MapComponent::init(Map::init_empty()));
        let entity = add_character(&mut ecs, Point::init(2, 2));
        assert_position(&ecs, &entity, Point::init(2, 2));

        let success = move_character(&mut ecs, entity, Point::init(2, 3));
        assert_eq!(true, success);

        assert_position(&ecs, &entity, Point::init(2, 3));
    }
    #[test]
    fn walk_into_non_characters() {
        let mut ecs = create_world();
        ecs.insert(MapComponent::init(Map::init_empty()));
        let entity = add_character(&mut ecs, Point::init(2, 2));
        ecs.create_entity()
            .with(PositionComponent::init(2, 3))
            .with(FieldComponent::init(255, 0, 0))
            .build();

        assert_position(&ecs, &entity, Point::init(2, 2));

        let success = move_character(&mut ecs, entity, Point::init(2, 3));
        assert_eq!(true, success);

        assert_position(&ecs, &entity, Point::init(2, 3));
    }

    #[test]
    fn unable_to_walk_into_unwalkable() {
        let mut ecs = create_world();
        let mut map = Map::init_empty();
        map.set_walkable(&Point::init(2, 3), false);
        ecs.insert(MapComponent::init(map));
        let entity = add_character(&mut ecs, Point::init(2, 2));
        assert_position(&ecs, &entity, Point::init(2, 2));

        let success = move_character(&mut ecs, entity, Point::init(2, 3));
        assert_eq!(false, success);

        assert_position(&ecs, &entity, Point::init(2, 2))
    }

    #[test]
    fn unable_to_walk_into_another() {
        let mut ecs = create_world();
        ecs.insert(MapComponent::init(Map::init_empty()));
        let entity = add_character(&mut ecs, Point::init(2, 2));
        ecs.create_entity()
            .with(PositionComponent::init(2, 3))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();

        assert_position(&ecs, &entity, Point::init(2, 2));

        let success = move_character(&mut ecs, entity, Point::init(2, 3));
        assert_eq!(false, success);

        assert_position(&ecs, &entity, Point::init(2, 2));
    }
    #[test]
    fn walk_off_map() {
        let mut ecs = create_world();
        ecs.insert(MapComponent::init(Map::init_empty()));
        let entity = add_character(&mut ecs, Point::init(13, 13));
        assert_position(&ecs, &entity, Point::init(13, 13));

        let success = move_character(&mut ecs, entity, Point::init(13, 14));
        assert_eq!(false, success);

        assert_position(&ecs, &entity, Point::init(13, 13));
    }
}
