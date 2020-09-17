use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::{ContextData, View};
use crate::after_image::{IconLoader, RenderCanvas, RenderContext};
use crate::atlas::BoxResult;

pub struct Frame {
    position: SDLPoint,
    frame: Texture,
}

impl Frame {
    pub fn init(position: SDLPoint, render_context: &RenderContext, loader: &IconLoader) -> BoxResult<Frame> {
        Ok(Frame {
            position,
            frame: loader.get(render_context, "info_frame.png")?,
        })
    }
}

impl View for Frame {
    fn render(&self, _: &World, canvas: &mut RenderCanvas, _frame: u64, _context: &ContextData) -> BoxResult<()> {
        canvas.copy(&self.frame, None, SDLRect::new(self.position.x(), self.position.y(), 271, 541))?;
        Ok(())
    }
}
