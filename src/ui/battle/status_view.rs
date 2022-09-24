use bevy_ecs::prelude::*;
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas},
};

use crate::core::{Player, Position};

pub fn draw_status(world: &mut World, canvas: &mut Canvas) {
    let query = &mut world.query_filtered::<&Position, With<Player>>();
    let position = query.single(world);

    let mut offset = 30.0;
    draw_status_line(canvas, &format!("Position: {}", position.position.origin), 875.0, &mut offset);
    offset = 230.0;
    draw_status_line(canvas, "Enemies:", 875.0, &mut offset);

    let query = &mut world.query_filtered::<&Position, Without<Player>>();
    for position in query.iter(world) {
        draw_status_line(canvas, &format!("Position: {}", position.position.origin), 875.0, &mut offset);
    }
}

fn draw_status_line(canvas: &mut Canvas, text: &str, x: f32, y: &mut f32) {
    canvas.draw(graphics::Text::new(text).set_font("default").set_scale(23.0), Vec2::new(x, *y));
    *y += 30.0;
}
