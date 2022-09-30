use bevy_ecs::prelude::*;

use super::{Player, Position, SizedPoint};

pub fn find_player(world: &mut World) -> Entity {
    let query = &mut world.query_filtered::<Entity, With<Player>>();
    let player = query.single(world);
    player
}

pub fn find_position(world: &mut World, entity: Entity) -> Option<SizedPoint> {
    world.get::<Position>(entity).map(|p| p.position)
}
