use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::super::IconLoader;
use super::View;
use crate::after_image::{RenderCanvas, RenderContext};
use crate::atlas::BoxResult;

pub struct SkillBarView {
    position: SDLPoint,
    views: Vec<Box<dyn View>>,
}

const BORDER_WIDTH: i32 = 5;
const ICON_SIZE: i32 = 44;
const ICON_COUNT: i32 = 15;

impl SkillBarView {
    pub fn init(render_context: &RenderContext, position: SDLPoint) -> BoxResult<SkillBarView> {
        let mut views: Vec<Box<dyn View>> = Vec::with_capacity(15);
        let icons = IconLoader::init(render_context, "spell")?;
        for i in 0..ICON_COUNT {
            let image = icons.get(render_context, test_skill_name(i))?;
            let view = SkillBarItemView::init(
                SDLPoint::new(BORDER_WIDTH + position.x + (ICON_SIZE + BORDER_WIDTH) * i, position.y + BORDER_WIDTH + 1),
                i as u32,
                image,
            )?;
            views.push(Box::from(view));
        }
        Ok(SkillBarView { position, views })
    }
}

fn test_skill_name(i: i32) -> &'static str {
    match i {
        0 => "SpellBook01_26.png",
        1 => "SpellBook01_07.png",
        2 => "SpellBook06_22.png",
        3 => "SpellBook06_118.png",
        4 => "SpellBookPage09_41.png",
        5 => "SpellBookPage09_06.png",
        6 => "SpellBook03_59.png",
        7 => "SpellBook06_05.png",
        8 => "SpellBookPage09_79.png",
        9 => "SpellBookPage09_112.png",
        10 => "SpellBookPage09_95.PNG",
        11 => "SpellBook06_92.png",
        12 => "SpellBook03_41.png",
        13 => "SpellBook03_46.png",
        14 => "SpellBook03_55.png",
        _ => "en_craft_80.png",
    }
}

impl View for SkillBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((22, 22, 22)));
        canvas.fill_rect(SDLRect::new(
            self.position.x,
            self.position.y,
            ((ICON_SIZE + BORDER_WIDTH) * ICON_COUNT + BORDER_WIDTH) as u32,
            (ICON_SIZE + BORDER_WIDTH * 2) as u32,
        ))?;

        for v in self.views.iter() {
            v.render(ecs, canvas, frame)?;
        }

        Ok(())
    }
}

pub struct SkillBarItemView {
    rect: SDLRect,
    index: u32,
    image: Texture,
}

impl SkillBarItemView {
    pub fn init(position: SDLPoint, index: u32, image: Texture) -> BoxResult<SkillBarItemView> {
        let rect = SDLRect::new(position.x, position.y, 44, 44);
        Ok(SkillBarItemView { rect, index, image })
    }
}

impl View for SkillBarItemView {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.copy(&self.image, SDLRect::new(0, 0, 256, 256), self.rect)?;
        Ok(())
    }
}
