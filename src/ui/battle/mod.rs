use bevy_ecs::prelude::*;
use ggez::{glam::Vec2, mint};

use crate::core::{Map, Player, Point};

pub mod battle_scene;
pub mod debug_overlay;

mod status_view;
pub use status_view::*;

mod messages;
pub use messages::*;

mod skillbar;
pub use skillbar::*;

mod overlay;
pub use overlay::*;

mod target_overlay;
pub use target_overlay::*;

// The map is placed at 16x16 but the first maptile is a ways off the corner
pub const MAP_IMAGE_POSITION: mint::Point2<f32> = mint::Point2 { x: 31.0, y: 31.0 };
pub const MAP_CORNER_X: f32 = 65.0;
pub const MAP_CORNER_Y: f32 = 65.0;
pub const TILE_SIZE: f32 = 56.0;

/// The upper left position of map point (x,y) on screen
pub fn screen_point_for_map_grid(x: f32, y: f32) -> Vec2 {
    let x = MAP_CORNER_X + x * TILE_SIZE;
    let y = MAP_CORNER_Y + y * TILE_SIZE;
    Vec2::new(x, y)
}

pub fn screen_to_map_position(x: f32, y: f32) -> Option<Point> {
    // First remove map offset
    let x = x - MAP_CORNER_X;
    let y = y - MAP_CORNER_Y;

    if x < 0.0 || y < 0.0 {
        return None;
    }

    // Now divide by grid position
    let x = x as u32 / TILE_SIZE as u32;
    let y = y as u32 / TILE_SIZE as u32;

    // Don't go off map
    if x >= Map::MAX_TILES as u32 || y >= Map::MAX_TILES as u32 {
        return None;
    }
    Some(Point::new(x, y))
}

pub fn get_player_entity(world: &mut World) -> Entity {
    let query = &mut world.query_filtered::<Entity, With<Player>>();
    let player = query.single(world);
    player
}
