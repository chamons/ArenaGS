use bevy_ecs::prelude::*;
use ggez::{graphics::Canvas, input::keyboard::KeyInput};
use winit::event::VirtualKeyCode;

use super::debug_overlay::DebugKind;
use super::debug_overlay::DebugOverlayRequest;
use super::*;
use crate::core::*;
use crate::ui::*;

#[no_mangle]
pub fn battle_update(_world: &mut World, _ctx: &mut ggez::Context) {}

#[no_mangle]
pub fn battle_draw(world: &mut World, ctx: &mut ggez::Context, canvas: &mut Canvas) {
    world.get_resource_mut::<Frame>().unwrap().current += 1;
    animation::advance_all_animations(world);

    draw_map(world, canvas);
    draw_status(world, canvas);
    message_draw(world, ctx, canvas);
    skillbar_draw(world, ctx, canvas);

    for (appearance, position) in &world.query::<(&Appearance, &Position)>().iter(world).collect::<Vec<_>>() {
        let mut render_position = screen_point_for_map_grid(position.origin().x, position.origin().y);

        render_position.x += (position.position.width as f32 * TILE_SIZE) / 2.0;
        render_position.y += (position.position.height as f32 * TILE_SIZE) / 2.0;

        let images = world.get_resource::<ImageCache>().unwrap();
        draw::render_sprite(canvas, render_position, appearance, images);
    }
}

#[no_mangle]
pub fn battle_key_up_event(world: &mut World, _ctx: &mut ggez::Context, input: KeyInput) {
    match input.keycode {
        Some(VirtualKeyCode::F1) => {
            world.get_resource_mut::<Scenes>().unwrap().push(SceneKind::DebugOverlay);
            world.insert_resource(DebugOverlayRequest::new(DebugKind::MapOverlay));
        }
        Some(VirtualKeyCode::D) => {
            let query = &world.query::<(Entity, &Appearance)>().iter(world).collect::<Vec<_>>();
            let first = query
                .iter()
                .filter(|(_, a)| a.kind == AppearanceKind::MaleBrownHairBlueBody)
                .map(|(e, _)| e)
                .next()
                .unwrap();
            world.send_event(SpriteAnimateActionEvent::new(*first, AnimationState::Cheer));
            let frame = world.get_resource::<Frame>().unwrap().current;
            world.send_event(NewMessageEvent::new(&format!("Dance Party: {}", frame)));
        }
        Some(VirtualKeyCode::PageUp) => world.send_event(ScrollMessageEvent::page_up()),
        Some(VirtualKeyCode::PageDown) => world.send_event(ScrollMessageEvent::page_down()),
        Some(VirtualKeyCode::End) => world.send_event(ScrollMessageEvent::scroll_to_end()),

        _ => {}
    }
}

#[no_mangle]
pub fn battle_draw_previous() -> bool {
    false
}

fn draw_map(world: &mut World, canvas: &mut Canvas) {
    let map = world.get_resource::<Map>().unwrap();
    let map_image = map.kind.filename().to_string();
    draw_image(canvas, world, &map_image, MAP_IMAGE_POSITION);
}

impl BackingImage for MapKind {
    fn filename(&self) -> &str {
        match self {
            MapKind::Ashlands => "/maps/ashlands/map1.png",
            MapKind::Beach => "/maps/beach/map1.png",
            MapKind::Desert => "/maps/desert/map1.png",
            MapKind::Ruins => "/maps/ruins/map1.png",
            MapKind::Winter => "/maps/winter/map1.png",
        }
    }
}
