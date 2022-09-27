use bevy_ecs::prelude::*;
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Drawable, Rect},
};

use crate::core::Log;

const LOG_LEFT: f32 = 875.0;
const LOG_WIDTH: f32 = 1280.0 - 875.0;
const LOG_TOP: f32 = 750.0;
const LOG_BOTTOM: f32 = 950.0;

pub fn message_draw(world: &mut World, ctx: &mut ggez::Context, canvas: &mut Canvas) {
    let log = world.get_resource::<Log>().unwrap();

    let mut y = LOG_TOP;

    println!("{}", log.index);
    for l in log.messages.iter().skip(log.index) {
        let mut end = graphics::Text::new(l);
        end.set_font("default").set_scale(23.0).set_bounds(Vec2::new(LOG_WIDTH, LOG_BOTTOM - y));
        if let Some(dimensions) = end.dimensions(ctx) {
            if y + dimensions.h < LOG_BOTTOM {
                canvas.draw(&end, Vec2::new(LOG_LEFT, y));
                y += dimensions.h;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}
