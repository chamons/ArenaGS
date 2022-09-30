use bevy_ecs::prelude::*;
use ggez::{
    self,
    graphics::{self, Canvas, Color, Rect},
    input::keyboard::KeyInput,
};
use winit::event::VirtualKeyCode;

use super::{screen_point_for_map_grid, screen_to_map_position, TILE_SIZE};
use crate::{
    core::{find_player, is_valid_target, Skill},
    ui::{Scenes, ScreenCoordinates, TILE_BORDER},
};

#[derive(Debug)]
pub struct TargetRequest {
    pub skill: Skill,
}

impl TargetRequest {
    pub fn new(skill: Skill) -> Self {
        TargetRequest { skill }
    }
}

#[no_mangle]
pub fn targeting_update(_world: &mut World, _ctx: &mut ggez::Context) {}

const TARGET_SIZE: Rect = Rect::new(TILE_BORDER, TILE_BORDER, TILE_SIZE - TILE_BORDER, TILE_SIZE - TILE_BORDER);

#[no_mangle]
pub fn targeting_draw(world: &mut World, ctx: &mut ggez::Context, canvas: &mut Canvas) {
    let mouse = ctx.mouse.position();
    world.resource_scope(|world, target: Mut<TargetRequest>| {
        let skill = &target.skill;

        if let Some(grid_rect) = screen_to_map_position(mouse.x, mouse.y) {
            let player = find_player(world);
            let color = if is_valid_target(world, player, skill, grid_rect) {
                Color::new(1.0, 1.0, 0.0, 0.75)
            } else {
                Color::new(1.0, 0.0, 0.0, 0.75)
            };

            let square = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), TARGET_SIZE, color).unwrap();
            let screen_point = screen_point_for_map_grid(grid_rect.x as f32, grid_rect.y as f32);
            canvas.draw(&square, screen_point);
        }
    });
}

#[no_mangle]
pub fn targeting_mouse_button_up_event(world: &mut World, ctx: &mut ggez::Context, button: ggez::event::MouseButton, x: f32, y: f32) {}

#[no_mangle]
pub fn targeting_key_up_event(world: &mut World, _ctx: &mut ggez::Context, input: KeyInput) {
    match input.keycode {
        Some(VirtualKeyCode::Escape) => {
            world.remove_resource::<TargetRequest>();
            world.get_resource_mut::<Scenes>().unwrap().pop();
        }
        _ => {}
    }
}

#[no_mangle]
pub fn targeting_draw_previous() -> bool {
    true
}
