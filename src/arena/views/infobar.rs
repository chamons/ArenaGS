use sdl2::rect::Point as SDLPoint;
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
        self.text.render_text("Hello World", self.position.x, self.position.y, canvas)?;

        Ok(())
    }
}

impl<'a> View for InfoBarView<'a> {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        self.render_character_info(canvas)?;
        Ok(())
    }
}
