use bevy_ecs::world::World;
use ggez::{graphics::Canvas, input::keyboard::KeyInput};
use winit::event::VirtualKeyCode;

use crate::ui::*;

#[cfg(debug_assertions)]
pub struct DebugOverlay {
    canceled: bool,
}

impl DebugOverlay {
    pub fn new() -> Self {
        DebugOverlay { canceled: false }
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

    fn draw(&mut self, _world: &mut World, _ctx: &mut ggez::Context, _canvas: &mut Canvas) {}

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
