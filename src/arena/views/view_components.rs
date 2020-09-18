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
    kind: FrameKind,
}

pub enum FrameKind {
    InfoBar,
    Log,
}

impl Frame {
    pub fn init(position: SDLPoint, render_context: &RenderContext, loader: &IconLoader, kind: FrameKind) -> BoxResult<Frame> {
        let image = match kind {
            FrameKind::InfoBar => "info_frame.png",
            FrameKind::Log => "log_frame.png",
        };
        Ok(Frame {
            position,
            frame: loader.get(render_context, image)?,
            kind,
        })
    }

    fn frame_size(&self) -> (u32, u32) {
        match self.kind {
            FrameKind::InfoBar => (271, 541),
            FrameKind::Log => (271, 227),
        }
    }
}

impl View for Frame {
    fn render(&self, _: &World, canvas: &mut RenderCanvas, _frame: u64, _context: &ContextData) -> BoxResult<()> {
        let frame_size = self.frame_size();
        canvas.copy(
            &self.frame,
            None,
            SDLRect::new(self.position.x(), self.position.y(), frame_size.0, frame_size.1),
        )?;
        Ok(())
    }
}
