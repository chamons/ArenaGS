use bevy_ecs::prelude::*;
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Drawable},
};

use crate::{
    core::{Log, LOG_ENTRIES_ON_SCREEN},
    ui::GAME_WIDTH,
};

const LOG_LEFT: f32 = 875.0;
const LOG_WIDTH: f32 = GAME_WIDTH - 875.0;
const LOG_TOP: f32 = 750.0;
const LOG_BOTTOM: f32 = 950.0;
const LOG_HEIGHT: f32 = LOG_BOTTOM - LOG_TOP;

pub fn message_draw(world: &mut World, ctx: &mut ggez::Context, canvas: &mut Canvas) {
    let log = world.get_resource::<Log>().unwrap();

    let mut y = LOG_TOP;

    let index = find_first_log_entry(log, ctx);
    for l in log.messages.iter().skip(index).take(LOG_ENTRIES_ON_SCREEN) {
        let text = configure_text_fragment(l);
        canvas.draw(&text, Vec2::new(LOG_LEFT, y));
        y += text.dimensions(ctx).unwrap().h;
    }
}

fn configure_text_fragment(text: &str) -> graphics::Text {
    let mut end = graphics::Text::new(text);
    end.set_font("default").set_scale(23.0).set_bounds(Vec2::new(LOG_WIDTH, LOG_HEIGHT));
    end
}

fn find_first_log_entry(log: &Log, ctx: &ggez::Context) -> usize {
    let mut remaining_height = LOG_HEIGHT;
    // Walk back from the last_index towards the front, measuring each line of text
    // until we have enough
    let message_count = log.messages.len();
    let last_index = log.last_index;
    for (i, line) in log.messages.iter().enumerate().rev().skip(message_count - last_index) {
        let text = configure_text_fragment(line);
        if let Some(dimensions) = text.dimensions(ctx) {
            remaining_height -= dimensions.h;
            if remaining_height == 0.0 {
                return i;
            } else if remaining_height < 0.0 {
                return i + 1;
            }
        } else {
            return i + 1;
        }
    }
    0
}
