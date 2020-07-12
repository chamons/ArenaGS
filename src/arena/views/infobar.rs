use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::View;
use crate::after_image::{RenderCanvas, TextRenderer};
use crate::atlas::BoxResult;

pub struct InfoBarView<'a> {
    position: SDLPoint,
    text: &'a TextRenderer<'a>,
}

impl<'a> InfoBarView<'a> {
    pub fn init(position: SDLPoint, text: &'a TextRenderer<'a>) -> BoxResult<InfoBarView> {
        Ok(InfoBarView { position, text })
    }
    fn render_character_info(&self, canvas: &mut RenderCanvas) -> BoxResult<()> {
        self.text.render_text("Info Bar", self.position.x, self.position.y, canvas)?;

        Ok(())
    }
}

impl<'a> View for InfoBarView<'a> {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((196, 196, 0)));
        canvas.fill_rect(SDLRect::new(self.position.x, self.position.y, 230, 400))?;
        self.render_character_info(canvas)?;

        Ok(())
    }
}
