use bevy_ecs::world::World;
use ggez::{
    event::MouseButton,
    glam::Vec2,
    graphics::{self, Canvas, Color, Rect},
    input::keyboard::KeyInput,
};
use winit::event::VirtualKeyCode;

use crate::{
    core::{Map, Point},
    ui::*,
};

use super::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum DebugKind {
    MapOverlay,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugOverlayRequest {
    kind: DebugKind,
}

impl DebugOverlayRequest {
    pub fn new(kind: DebugKind) -> Self {
        DebugOverlayRequest { kind }
    }
}

#[no_mangle]
pub fn debug_update(_world: &mut World, _ctx: &mut ggez::Context) {}

#[no_mangle]
pub fn debug_draw(world: &mut World, ctx: &mut ggez::Context, canvas: &mut Canvas) {
    let overlay_kind = world.get_resource::<DebugOverlayRequest>().unwrap().kind;

    canvas.draw(
        graphics::Text::new(format!("Debug: {:?}", overlay_kind)).set_font("default").set_scale(18.0),
        Vec2::new(10.0, 10.0),
    );

    match overlay_kind {
        DebugKind::MapOverlay => {
            let square_size = Rect::new(TILE_BORDER, TILE_BORDER, TILE_SIZE - TILE_BORDER, TILE_SIZE - TILE_BORDER);
            let red_square = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), square_size, Color::new(0.8, 0.1, 0.1, 0.5)).unwrap();
            let green_square = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), square_size, Color::new(0.1, 0.8, 0.1, 0.5)).unwrap();

            draw_map_grid(canvas, ctx);

            let map = world.get_resource::<Map>().unwrap();
            for x in 0..Map::MAX_TILES as u32 {
                for y in 0..Map::MAX_TILES as u32 {
                    let grid_rect = screen_point_for_map_grid(x as f32, y as f32);
                    if map.is_walkable(&Point::new(x, y)) {
                        canvas.draw(&green_square, grid_rect);
                    } else {
                        canvas.draw(&red_square, grid_rect);
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub fn debug_mouse_button_up_event(world: &mut World, ctx: &mut ggez::Context, button: ggez::event::MouseButton, x: f32, y: f32) {
    let screen_coordinate = world.get_resource::<ScreenCoordinates>().unwrap();
    let (x, y) = logical_mouse_position(ctx, screen_coordinate, x, y);

    if button == MouseButton::Left {
        if let Some(point) = screen_to_map_position(x, y) {
            let mut map = world.get_resource_mut::<Map>().unwrap();
            let was_walkable = map.is_walkable(&point);
            map.set_walkable(&point, !was_walkable);
        }
    }
}

#[no_mangle]
pub fn debug_key_up_event(world: &mut World, _ctx: &mut ggez::Context, input: KeyInput) {
    match input.keycode {
        Some(VirtualKeyCode::F1) => {
            world.get_resource_mut::<Scenes>().unwrap().pop();
            world.remove_resource::<DebugOverlayRequest>();
        }
        _ => {}
    }
}

#[no_mangle]
pub fn debug_draw_previous() -> bool {
    true
}
