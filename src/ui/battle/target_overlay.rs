use bevy_ecs::prelude::*;
use ggez::{
    self,
    graphics::{self, Canvas, Color, Rect},
    input::keyboard::KeyInput,
};
use winit::event::VirtualKeyCode;

use super::{screen_point_for_map_grid, screen_to_map_position, TILE_SIZE};
use crate::{
    core::{find_player, find_position, is_valid_target, Point, Skill},
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
    let (x, y) = world.get_resource::<ScreenCoordinates>().unwrap().logical_mouse_position(ctx, mouse.x, mouse.y);

    world.resource_scope(|world, target: Mut<TargetRequest>| {
        let skill = &target.skill;

        if let Some(cursor_point_on_map) = screen_to_map_position(x, y) {
            let player = find_player(world);
            let color = if is_valid_target(world, player, skill, cursor_point_on_map) {
                Color::new(1.0, 1.0, 0.0, 0.75)
            } else {
                Color::new(1.0, 0.0, 0.0, 0.75)
            };

            let square = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), TARGET_SIZE, color).unwrap();
            let screen_point = screen_point_for_map_grid(cursor_point_on_map.x as f32, cursor_point_on_map.y as f32);
            canvas.draw(&square, screen_point);

            if skill.show_trail() {
                if let Some(points) = find_position(world, player).unwrap().line_to(cursor_point_on_map) {
                    draw_line(&points, color, ctx, canvas);
                }
            }
        }
    });
}

fn draw_line(points: &[Point], color: Color, ctx: &mut ggez::Context, canvas: &mut Canvas) {
    let square = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), TARGET_SIZE, color).unwrap();

    for p in points.iter().skip(1) {
        let screen_point = screen_point_for_map_grid(p.x as f32, p.y as f32);
        canvas.draw(&square, screen_point);
    }
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
