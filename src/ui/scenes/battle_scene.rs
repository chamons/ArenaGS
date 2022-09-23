use bevy_ecs::prelude::*;
use ggez::{graphics::Canvas, input::keyboard::KeyInput};
use winit::event::VirtualKeyCode;

use super::*;
use crate::core::*;
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
    fn update(&mut self, world: &mut World, ctx: &mut ggez::Context) -> SceneSwitch<World> {
        if self.request_debug {
            self.request_debug = false;
            let scale = world.get_resource::<ScreenScale>().unwrap().scale;
            return SceneSwitch::Push(Box::new(DebugOverlay::new(ctx, scale)));
        }
        SceneSwitch::None
    }

    fn draw(&mut self, world: &mut World, _ctx: &mut ggez::Context, canvas: &mut Canvas) {
        world.get_resource_mut::<Frame>().unwrap().current += 1;
        advance_animations(world);

        draw_map(world, canvas);

        let scale = world.get_resource::<ScreenScale>().unwrap().scale;
        for (appearance, position) in &world.query::<(&Appearance, &Position)>().iter(world).collect::<Vec<_>>() {
            let mut render_position = screen_point_for_map_grid(position.origin().x, position.origin().y, scale);

            render_position.x += (position.position.width as f32 * TILE_SIZE * scale) / 2.0;
            render_position.y += (position.position.height as f32 * TILE_SIZE * scale) / 2.0;

            let images = world.get_resource::<ImageCache>().unwrap();
            sprite::draw(canvas, render_position, appearance, scale, images);
        }
    }

    fn key_up_event(&mut self, _world: &mut World, _ctx: &mut ggez::Context, input: KeyInput) {
        match input.keycode {
            Some(VirtualKeyCode::F1) => self.request_debug = true,
            _ => {}
        }
    }
}

fn draw_map(world: &mut World, canvas: &mut Canvas) {
    let map = world.get_resource::<Map>().unwrap();
    let map_image = match map.kind {
        MapKind::Ashlands => "/maps/ashlands/map1.png",
        MapKind::Beach => "/maps/beach/map1.png",
        MapKind::Desert => "/maps/desert/map1.png",
        MapKind::Ruins => "/maps/ruins/map1.png",
        MapKind::Winter => "/maps/winter/map1.png",
    };
    draw_image(canvas, world, map_image, MAP_IMAGE_POSITION);
}

fn advance_animations(world: &mut World) {
    let mut query = world.query::<&mut Appearance>();

    for mut appearance in query.iter_mut(world) {
        if appearance.animation.is_none() {
            appearance.animation = Some(appearance.create_animation())
        }

        if let Some(animation) = &mut appearance.animation {
            animation.advance_and_maybe_reverse(1.0);
        }
    }
}
