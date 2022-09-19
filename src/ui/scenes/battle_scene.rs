use bevy_ecs::world::World;
use ggez::{graphics::Canvas, input::keyboard::KeyInput, mint};
use winit::event::VirtualKeyCode;

use super::*;
use crate::ui::*;

pub struct BattleScene {
    request_debug: bool,
}

impl BattleScene {
    pub fn new() -> Self {
        BattleScene { request_debug: false }
    }
}

impl Scene<World> for BattleScene {
    fn update(&mut self, _world: &mut World, ctx: &mut ggez::Context) -> SceneSwitch<World> {
        if self.request_debug {
            self.request_debug = false;
            return SceneSwitch::Push(Box::new(DebugOverlay::new(ctx)));
        }
        SceneSwitch::None
    }

    fn draw(&mut self, world: &mut World, _ctx: &mut ggez::Context, canvas: &mut Canvas) {
        draw_image(canvas, world, "/maps/beach/map1.png", MAP_IMAGE_POSITION);

        let _map = world.get_resource::<crate::core::map::Map>().unwrap();
    }

    fn key_up_event(&mut self, _world: &mut World, _ctx: &mut ggez::Context, input: KeyInput) {
        match input.keycode {
            Some(VirtualKeyCode::F1) => self.request_debug = true,
            _ => {}
        }
    }
}
