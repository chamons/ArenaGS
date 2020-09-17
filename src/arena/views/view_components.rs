use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::{ContextData, View};
use crate::after_image::{RenderCanvas};
use crate::atlas::{BoxResult};

pub struct Frame {
    position: SDLPoint
}

impl Frame {
    pub fn init(position: SDLPoint) -> BoxResult<Frame> {
        Ok(Frame { position})
    }
}

impl View for Frame {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64, _context: &ContextData) -> BoxResult<()> {
        Ok(())
    }
}
