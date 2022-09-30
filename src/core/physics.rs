use bevy_ecs::prelude::*;

use super::{find_player, Character, Map, Point, Position};

// Is an area clear of all elements with PositionComponent and IsCharacterComponent _except_ the invoker (if)
pub fn is_area_clear_of_others(world: &mut World, area: &[Point], invoker: Entity) -> bool {
    is_area_clear(world, area, Some(invoker))
}

pub fn is_area_clear(world: &mut World, area: &[Point], invoker: Option<Entity>) -> bool {
    world.resource_scope(|world, map: Mut<Map>| {
        let mut query = world.query_filtered::<(Entity, &Position), With<Character>>();

        for (entity, position) in query.iter(world) {
            for p in area.iter() {
                if !p.in_bounds() || !map.is_walkable(&p) {
                    return false;
                }
                if invoker != Some(entity) && position.position.contains_point(&p) {
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
