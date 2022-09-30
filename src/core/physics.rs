use bevy_ecs::prelude::*;

use super::Point;

// Is an area clear of all elements with PositionComponent and IsCharacterComponent _except_ the invoker (if)
pub fn is_area_clear_of_others(ecs: &World, area: &[Point], invoker: Entity) -> bool {
    is_area_clear(ecs, area, Some(invoker))
}

pub fn is_area_clear(ecs: &World, area: &[Point], invoker: Option<Entity>) -> bool {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let positions = ecs.read_storage::<PositionComponent>();
    let is_characters = ecs.read_storage::<IsCharacterComponent>();
    let map = &ecs.read_resource::<MapComponent>().map;

    for (entity, position, _) in (&entities, &positions, &is_characters).join() {
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
}

pub fn find_character_at_location(ecs: &World, area: Point) -> Option<Entity> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let positions = ecs.read_storage::<PositionComponent>();
    let is_characters = ecs.read_storage::<IsCharacterComponent>();

    for (entity, position, _) in (&entities, &positions, &is_characters).join() {
        if position.position.contains_point(&area) {
            return Some(entity);
        }
    }
    None
}

pub fn is_player_or_ally(ecs: &World, entity: Entity) -> bool {
    ecs.read_storage::<PlayerComponent>().get(entity).is_some() || ecs.read_storage::<PlayerAlly>().get(entity).is_some()
}
