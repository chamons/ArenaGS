use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::View;
use crate::after_image::{FontColor, FontSize, RenderCanvas, TextRenderer};
use crate::atlas::BoxResult;
use crate::clash::LogComponent;

const LOG_COUNT: usize = 10;

pub struct LogView<'a> {
    position: SDLPoint,
    text: &'a TextRenderer<'a>,
}

impl<'a> LogView<'a> {
    pub fn init(position: SDLPoint, text: &'a TextRenderer<'a>) -> BoxResult<LogView> {
        Ok(LogView { position, text })
    }

    fn render_log(&self, ecs: &World, canvas: &mut RenderCanvas) -> BoxResult<()> {
        let log = ecs.read_resource::<LogComponent>();
        for (i, entry) in log.get(log.index, LOG_COUNT).iter().enumerate() {
            self.text.render_text(
                entry,
                self.position.x,
                self.position.y + (i as i32 * 30),
                canvas,
                FontSize::Small,
                FontColor::Black,
            )?;
        }

        Ok(())
    }
}

impl<'a> View for LogView<'a> {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 196, 196)));
        canvas.fill_rect(SDLRect::new(self.position.x, self.position.y, 230, 300))?;
        self.render_log(ecs, canvas)?;

        Ok(())
    }
}
