use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
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
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((196, 0, 196)));

        canvas.fill_rect(SDLRect::new(self.position.x, self.position.y, 800, 44))?;
        Ok(())
    }
}
