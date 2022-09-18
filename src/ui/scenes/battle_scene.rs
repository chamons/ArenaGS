use bevy_ecs::world::World;
use ggez::{
    graphics::{Canvas, DrawParam, Transform},
    input::keyboard::KeyInput,
    mint,
};
use winit::event::VirtualKeyCode;

use crate::ui::*;

pub struct BattleScene {
    request_debug: bool,
}

impl BattleScene {
    pub fn new() -> Self {
        BattleScene { request_debug: false }
    }
}

fn get_image_draw_params(world: &mut World) -> DrawParam {
    let scale = world.get_resource::<ScreenScale>().unwrap();
    DrawParam {
        transform: Transform::Values {
            dest: mint::Point2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            scale: mint::Vector2 {
                x: scale.scale as f32,
                y: scale.scale as f32,
            },
            offset: mint::Point2 { x: 0.0, y: 0.0 },
        },
        ..Default::default()
    }
}

impl Scene<World> for BattleScene {
    fn update(&mut self, _world: &mut World, _ctx: &mut ggez::Context) -> SceneSwitch<World> {
        if self.request_debug {
            self.request_debug = false;
            return SceneSwitch::Push(Box::new(DebugOverlay::new()));
        }
        SceneSwitch::None
    }

    fn draw(&mut self, world: &mut World, _ctx: &mut ggez::Context, canvas: &mut Canvas) {
        let image_draw_params = get_image_draw_params(world);

        let images = world.get_resource::<crate::ui::ImageCache>().unwrap();
        canvas.draw(images.get("/maps/beach/map1.png"), image_draw_params);

        let _map = world.get_resource::<crate::core::map::Map>().unwrap();
    }

    fn key_up_event(&mut self, _world: &mut World, _ctx: &mut ggez::Context, input: KeyInput) {
        match input.keycode {
            Some(VirtualKeyCode::F1) => self.request_debug = true,
            _ => {}
        }
    }
}
