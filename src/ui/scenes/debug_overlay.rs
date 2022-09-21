use bevy_ecs::world::World;
use ggez::{
    event::MouseButton,
    glam::Vec2,
    graphics::{self, Canvas, Color, Mesh, Rect},
    input::keyboard::KeyInput,
};
use winit::event::VirtualKeyCode;

use crate::{
    core::{map::Map, utils::Point},
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
    pub fn new(ctx: &mut ggez::Context) -> Self {
        // Offset each debug tile overlay for a grid border
        let square_size = Rect::new(TILE_BORDER, TILE_BORDER, TILE_SIZE - TILE_BORDER, TILE_SIZE - TILE_BORDER);
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
        canvas.draw(
            graphics::Text::new(format!("Debug: {:?}", self.overlay_kind))
                .set_font("default")
                .set_scale(18.0),
            Vec2::new(10.0, 10.0),
        );

        match self.overlay_kind {
            DebugKind::MapOverlay => {
                draw_map_grid(canvas, ctx);

                let map = world.get_resource::<crate::core::map::Map>().unwrap();
                for x in 0..Map::MAX_TILES as u32 {
                    for y in 0..Map::MAX_TILES as u32 {
                        let grid_rect = screen_point_for_map_grid(x, y);
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

    fn mouse_button_up_event(&mut self, world: &mut World, _ctx: &mut ggez::Context, button: ggez::event::MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            if let Some(point) = screen_to_map_position(x, y) {
                let mut map = world.get_resource_mut::<crate::core::map::Map>().unwrap();
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
