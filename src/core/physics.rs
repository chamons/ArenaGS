use bevy_ecs::prelude::*;

use super::{find_player, Character, Map, Point, Position};

pub fn is_area_clear_of_others(world: &mut World, area: &[Point], invoker: Option<Entity>) -> bool {
    world.resource_scope(|world, map: Mut<Map>| {
        let mut query = world.query_filtered::<(Entity, &Position), With<Character>>();

        for (entity, position) in query.iter(world) {
            for p in area.iter() {
                if !p.in_bounds() || !map.is_walkable(p) {
                    return false;
                }
                if invoker != Some(entity) && position.position.contains_point(p) {
                    return false;
                }
            }
        }

        true
    })
}

pub fn find_character_at_location(world: &mut World, area: Point) -> Option<Entity> {
    let mut query = world.query_filtered::<(Entity, &Position), With<Character>>();

    for (entity, position) in query.iter(world) {
        if position.position.contains_point(&area) {
            return Some(entity);
        }
    }
    None
}

pub fn is_player_or_ally(world: &mut World, entity: Entity) -> bool {
    // TODO - Ally
    find_player(world) == entity
}

#[cfg(test)]
mod tests {
    use crate::core::{MapKind, Player};

    use super::*;

    #[test]
    fn find_characters() {
        let mut world = World::new();
        let first = world.spawn().insert(Character).insert(Position::new_sized(6, 6, 2, 2)).id();
        let second = world.spawn().insert(Character).insert(Position::new(3, 3)).id();
        assert_eq!(Some(first), find_character_at_location(&mut world, Point::new(6, 6)));
        assert_eq!(Some(first), find_character_at_location(&mut world, Point::new(7, 6)));
        assert_eq!(Some(second), find_character_at_location(&mut world, Point::new(3, 3)));
        assert_eq!(None, find_character_at_location(&mut world, Point::new(3, 4)));
    }

    #[test]
    fn is_player() {
        let mut world = World::new();
        let first = world.spawn().insert(Character).insert(Player).id();
        let second = world.spawn().insert(Character).id();
        assert!(is_player_or_ally(&mut world, first));
        assert!(!is_player_or_ally(&mut world, second));
    }

    #[test]
    fn area_clear() {
        let mut world = World::new();
        let first = world.spawn().insert(Character).insert(Position::new_sized(6, 6, 2, 2)).id();
        let second = world.spawn().insert(Character).insert(Position::new(3, 3)).id();
        let mut map = Map::empty(MapKind::Ashlands);
        map.set_walkable(&Point::new(2, 2), false);
        map.set_walkable(&Point::new(2, 4), false);
        world.insert_resource(map);

        assert!(is_area_clear_of_others(&mut world, &[Point::new(2, 3)], None));
        assert!(!is_area_clear_of_others(&mut world, &[Point::new(3, 3)], None));
        assert!(is_area_clear_of_others(&mut world, &[Point::new(3, 3)], Some(second)));
        assert!(!is_area_clear_of_others(&mut world, &[Point::new(6, 7)], Some(second)));
        assert!(is_area_clear_of_others(&mut world, &[Point::new(6, 7)], Some(first)));
        assert!(!is_area_clear_of_others(&mut world, &[Point::new(2, 2)], None));
        assert!(!is_area_clear_of_others(&mut world, &[Point::new(2, 4)], None));
        assert!(!is_area_clear_of_others(&mut world, &[Point::new(13, 14)], None));
    }
}
