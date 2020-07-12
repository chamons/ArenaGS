use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::View;
use crate::after_image::RenderCanvas;
use crate::atlas::BoxResult;

pub struct SkillBarView {
    position: SDLPoint,
}

impl SkillBarView {
    pub fn init(position: SDLPoint) -> BoxResult<SkillBarView> {
        Ok(SkillBarView { position })
    }
}

impl View for SkillBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        Ok(())
    }
}
