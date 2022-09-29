use bevy_ecs::world::World;
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color, DrawParam, Rect, Transform},
    mint,
};

use crate::core::{Map, Point};

pub mod battle_scene;
pub mod debug_overlay;

mod status_view;
pub use status_view::*;

mod messages;
pub use messages::*;

mod skillbar;
pub use skillbar::*;

use super::ScreenCoordinates;

// The map is placed at 16x16 but the first maptile is a ways off the corner
pub const MAP_IMAGE_POSITION: mint::Point2<f32> = mint::Point2 { x: 31.0, y: 31.0 };
pub const MAP_CORNER_X: f32 = 65.0;
pub const MAP_CORNER_Y: f32 = 65.0;
pub const TILE_SIZE: f32 = 56.0;

/// The upper left position of map point (x,y) on screen
pub fn screen_point_for_map_grid(x: u32, y: u32) -> Vec2 {
    let x = MAP_CORNER_X + (x as f32) * TILE_SIZE;
    let y = MAP_CORNER_Y + (y as f32) * TILE_SIZE;
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

fn draw_image(canvas: &mut Canvas, world: &mut World, image: &str, position: mint::Point2<f32>) {
    let images = world.get_resource::<crate::ui::ImageCache>().unwrap();

    canvas.draw(images.get(image), get_image_draw_params(position));
}

pub const GRID_COLOR: Color = Color::new(196.0 / 255.0, 196.0 / 255.0, 196.0 / 255.0, 1.0);
pub const TILE_BORDER: f32 = 2.0;

fn draw_map_grid(canvas: &mut Canvas, ctx: &mut ggez::Context) {
    let map_width = Map::MAX_TILES as f32 * TILE_SIZE;
    let grid_horz_edge = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect::new(0.0, 0.0, map_width + 1.0, TILE_BORDER), GRID_COLOR).unwrap();
    canvas.draw(&grid_horz_edge, Vec2::new(MAP_CORNER_X, MAP_CORNER_Y));
    canvas.draw(&grid_horz_edge, Vec2::new(MAP_CORNER_X, MAP_CORNER_Y + map_width));

    let grid_vert_edge = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect::new(0.0, 0.0, TILE_BORDER, map_width + 1.0), GRID_COLOR).unwrap();
    canvas.draw(&grid_vert_edge, Vec2::new(MAP_CORNER_X, MAP_CORNER_Y));
    canvas.draw(&grid_vert_edge, Vec2::new(MAP_CORNER_X + map_width, MAP_CORNER_Y));

    for x in 0..Map::MAX_TILES {
        canvas.draw(&grid_vert_edge, Vec2::new(MAP_CORNER_X + (x as f32 * TILE_SIZE), MAP_CORNER_Y));
        for y in 0..Map::MAX_TILES {
            canvas.draw(&grid_horz_edge, Vec2::new(MAP_CORNER_X, MAP_CORNER_Y + (y as f32 * TILE_SIZE)));
        }
    }
}

fn get_image_draw_params(dest: mint::Point2<f32>) -> DrawParam {
    DrawParam {
        transform: Transform::Values {
            dest,
            rotation: 0.0,
            scale: mint::Vector2 { x: 1.0, y: 1.0 },
            offset: mint::Point2 { x: 0.0, y: 0.0 },
        },
        ..Default::default()
    }
}

fn logical_mouse_position(ctx: &mut ggez::Context, screen_coordinate: &ScreenCoordinates, x: f32, y: f32) -> (f32, f32) {
    let screen_rect = screen_coordinate.rect;
    let size = ctx.gfx.window().inner_size();
    let pos_x = (x / (size.width as f32)) * screen_rect.w + screen_rect.x;
    let pos_y = (y / (size.height as f32)) * screen_rect.h + screen_rect.y;
    (pos_x, pos_y)
}