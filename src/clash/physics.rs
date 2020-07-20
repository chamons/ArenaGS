use super::*;

use specs::prelude::*;

pub fn move_character(ecs: &mut World, entity: Entity, new: &Point) -> bool {
    let map = &ecs.read_resource::<MapComponent>().map;
    //if map.is_walkable(new) {}

    let mut positions = ecs.write_storage::<PositionComponent>();
    let mut position = &mut positions.get_mut(entity).unwrap();
    position.x = new.x;
    position.y = new.y;
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
        assert_eq!(position.x, expected.x);
        assert_eq!(position.y, expected.y);
    }

    #[test]
    fn walk_into_clear() {
        let mut ecs = create_world();
        ecs.insert(MapComponent::init(Map::init_empty()));
        let entity = add_character(&mut ecs, Point::init(2, 2));
        assert_position(&ecs, &entity, Point::init(2, 2));
        let success = move_character(&mut ecs, entity, &Point::init(2, 3));
        assert_eq!(true, success);
        assert_position(&ecs, &entity, Point::init(2, 3));
    }
    #[test]
    fn walk_into_non_characters() {
        let mut ecs = create_world();
    }

    #[test]
    fn unable_to_walk_into_unwalkable() {
        let mut ecs = create_world();
    }

    #[test]
    fn unable_to_walk_into_another() {
        let mut ecs = create_world();
    }
}
