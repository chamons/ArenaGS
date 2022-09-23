use bevy_ecs::world::World;
use ggez::{
    event::MouseButton,
    glam::Vec2,
    graphics::{self, Canvas, Color, Mesh, Rect},
    input::keyboard::KeyInput,
};
use winit::event::VirtualKeyCode;

use crate::{
    core::{Map, Point},
    ui::*,
};

use super::draw_map_grid;

#[derive(Debug)]
pub enum DebugKind {
    MapOverlay,
}

#[cfg(debug_assertions)]
pub struct DebugOverlay {
    canceled: bool,
    overlay_kind: DebugKind,
    red_square: Mesh,
    green_square: Mesh,
}

impl DebugOverlay {
    pub fn new(ctx: &mut ggez::Context, scale: f32) -> Self {
        // Offset each debug tile overlay for a grid border
        let square_size = Rect::new(TILE_BORDER, TILE_BORDER, scale * (TILE_SIZE - TILE_BORDER), scale * (TILE_SIZE - TILE_BORDER));
        let red_square = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), square_size, Color::new(0.8, 0.1, 0.1, 0.5)).unwrap();
        let green_square = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), square_size, Color::new(0.1, 0.8, 0.1, 0.5)).unwrap();

        DebugOverlay {
            canceled: false,
            overlay_kind: DebugKind::MapOverlay,
            red_square,
            green_square,
        }
    }
}

impl Scene<World> for DebugOverlay {
    fn update(&mut self, _world: &mut World, _ctx: &mut ggez::Context) -> SceneSwitch<World> {
        if self.canceled {
            SceneSwitch::Pop
        } else {
            SceneSwitch::None
        }
    }

    fn draw(&mut self, world: &mut World, ctx: &mut ggez::Context, canvas: &mut Canvas) {
        let scale = world.get_resource::<ScreenScale>().unwrap().scale;

        canvas.draw(
            graphics::Text::new(format!("Debug: {:?}", self.overlay_kind))
                .set_font("default")
                .set_scale(18.0 * scale),
            Vec2::new(10.0 * scale, 10.0 * scale),
        );

        match self.overlay_kind {
            DebugKind::MapOverlay => {
                draw_map_grid(canvas, ctx, scale);

                let map = world.get_resource::<Map>().unwrap();
                for x in 0..Map::MAX_TILES as u32 {
                    for y in 0..Map::MAX_TILES as u32 {
                        let grid_rect = screen_point_for_map_grid(x, y, scale);
                        if map.is_walkable(&Point::new(x, y)) {
                            canvas.draw(&self.green_square, grid_rect);
                        } else {
                            canvas.draw(&self.red_square, grid_rect);
                        }
                    }
                }
            }
        }
    }

    fn mouse_motion_event(&mut self, _world: &mut World, ctx: &mut ggez::Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        let (x, y) = logical_mouse_position(ctx, x, y);
        println!("{},{}", x.round(), y.round());
    }

    fn mouse_button_up_event(&mut self, world: &mut World, ctx: &mut ggez::Context, button: ggez::event::MouseButton, x: f32, y: f32) {
        let (x, y) = logical_mouse_position(ctx, x, y);

        let scale = ctx.gfx.window().scale_factor();
        if button == MouseButton::Left {
            if let Some(point) = screen_to_map_position(x, y, 1.0) {
                let mut map = world.get_resource_mut::<Map>().unwrap();
                let was_walkable = map.is_walkable(&point);
                map.set_walkable(&point, !was_walkable);
            }
        }
    }

    fn key_up_event(&mut self, _world: &mut World, _ctx: &mut ggez::Context, input: KeyInput) {
        match input.keycode {
            Some(VirtualKeyCode::F1) => self.canceled = true,
            _ => {}
        }
    }

    fn draw_previous(&self) -> bool {
        true
    }
}

fn logical_mouse_position(ctx: &mut ggez::Context, x: f32, y: f32) -> (f32, f32) {
    let screen_rect = graphics::Rect::new(0.0, 0.0, 1280.0, 960.0);
    let size = ctx.gfx.window().inner_size();
    let pos_x = (x / (size.width as f32)) * screen_rect.w + screen_rect.x;
    let pos_y = (y / (size.height as f32)) * screen_rect.h + screen_rect.y;
    (pos_x, pos_y)
}
