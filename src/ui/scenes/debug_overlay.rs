use bevy_ecs::world::World;
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color, Mesh, Rect},
    input::keyboard::KeyInput,
};
use winit::event::VirtualKeyCode;

use crate::{core::map::Map, ui::*};

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
        let square_size = Rect::new(0.0, 0.0, TILE_SIZE, TILE_SIZE);
        let red_square = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), square_size, Color::new(1.0, 0.0, 0.0, 0.5)).unwrap();
        let green_square = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), square_size, Color::new(0.0, 1.0, 0.0, 0.5)).unwrap();

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

    fn draw(&mut self, _world: &mut World, _ctx: &mut ggez::Context, canvas: &mut Canvas) {
        canvas.draw(
            graphics::Text::new(format!("Debug: {:?}", self.overlay_kind))
                .set_font("default")
                .set_scale(18.0),
            Vec2::new(10.0, 10.0),
        );

        match self.overlay_kind {
            DebugKind::MapOverlay => {
                let mut flip = false;
                for x in 0..Map::MAX_TILES {
                    for y in 0..Map::MAX_TILES {
                        let grid_rect = screen_point_for_map_grid(x, y);
                        if flip {
                            canvas.draw(&self.red_square, grid_rect);
                        } else {
                            canvas.draw(&self.green_square, grid_rect);
                        }
                        flip = !flip;
                    }
                }
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
