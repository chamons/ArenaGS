use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::{ContextData, HitTestResult, View};
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
    Map,
}

impl Frame {
    pub fn init(position: SDLPoint, render_context: &RenderContext, loader: &IconLoader, kind: FrameKind) -> BoxResult<Frame> {
        let image = match kind {
            FrameKind::InfoBar => "info_frame.png",
            FrameKind::Log => "log_frame.png",
            FrameKind::Map => "map_frame.png",
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
            FrameKind::Map => (753, 768),
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

type ButtonHandler = dyn Fn() -> Option<HitTestResult>;

pub struct Button {
    frame: SDLRect,
    background: Texture,
    handler: Box<ButtonHandler>,
}

impl Button {
    pub fn init(frame: SDLRect, background: Texture, handler: impl Fn() -> Option<HitTestResult> + 'static) -> BoxResult<Button> {
        Ok(Button {
            frame,
            background,
            handler: Box::new(handler),
        })
    }
}

impl View for Button {
    fn render(&self, _: &World, canvas: &mut RenderCanvas, _frame: u64, _context: &ContextData) -> BoxResult<()> {
        canvas.copy(&self.background, None, self.frame)?;
        Ok(())
    }

    fn hit_test(&self, _ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        if self.frame.contains_point(SDLPoint::new(x, y)) {
            (self.handler)()
        } else {
            None
        }
    }
}
