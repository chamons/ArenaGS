use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::super::IconLoader;
use super::View;
use crate::after_image::{RenderCanvas, RenderContext};
use crate::atlas::BoxResult;

pub struct SkillBarView {
    position: SDLPoint,
    views: Vec<Box<dyn View>>,
    icons: IconLoader,
}

impl SkillBarView {
    pub fn init(render_context: &RenderContext, position: SDLPoint) -> BoxResult<SkillBarView> {
        let mut views: Vec<Box<dyn View>> = Vec::with_capacity(15);
        for i in 0..15 {
            let view = SkillBarItemView::init(SDLPoint::new(6 + position.x + 50 * i, position.y + SKILL_BAR_BORDER_Y as i32), i as u32)?;
            views.push(Box::from(view));
        }
        let icons = IconLoader::init(render_context)?;
        Ok(SkillBarView { position, views, icons })
    }
}

const SKILL_BAR_BORDER_Y: u32 = 5;

impl View for SkillBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((35, 35, 35)));
        canvas.fill_rect(SDLRect::new(self.position.x, self.position.y, 50 * 15 + 6, 44 + SKILL_BAR_BORDER_Y * 2))?;

        for v in self.views.iter() {
            v.render(ecs, canvas, frame)?;
        }

        Ok(())
    }
}

pub struct SkillBarItemView {
    position: SDLPoint,
    index: u32,
}

impl SkillBarItemView {
    pub fn init(position: SDLPoint, index: u32) -> BoxResult<SkillBarItemView> {
        Ok(SkillBarItemView { position, index })
    }
}

impl View for SkillBarItemView {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((196, 0, 0)));
        canvas.fill_rect(SDLRect::new(self.position.x, self.position.y, 44, 44))?;
        Ok(())
    }
}
