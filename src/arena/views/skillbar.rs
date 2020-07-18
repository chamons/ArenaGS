use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::super::IconLoader;
use super::{HitTestResult, View};
use crate::after_image::{FontColor, FontSize, RenderCanvas, RenderContext, TextRenderer};
use crate::atlas::BoxResult;

pub struct SkillBarView {
    position: SDLPoint,
    views: Vec<SkillBarItemView>,
}

const BORDER_WIDTH: i32 = 5;
const ICON_SIZE: i32 = 44;
const ICON_COUNT: u32 = 10;

impl SkillBarView {
    pub fn init(render_context: &RenderContext, position: SDLPoint, text: &TextRenderer) -> BoxResult<SkillBarView> {
        let mut views = Vec::with_capacity(15);
        let icons = IconLoader::init(render_context, "spell")?;
        for i in 0..ICON_COUNT {
            let image = icons.get(render_context, test_skill_name(i))?;
            let hotkey = text.render_texture(&render_context.canvas, &i.to_string(), FontSize::Bold, FontColor::White)?;
            let view = SkillBarItemView::init(
                SDLPoint::new(BORDER_WIDTH + position.x + (ICON_SIZE + BORDER_WIDTH) * i as i32, position.y + BORDER_WIDTH + 1),
                i as u32,
                image,
                hotkey,
            )?;
            views.push(view);
        }
        Ok(SkillBarView { position, views })
    }
}

pub fn test_skill_name(i: u32) -> &'static str {
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
        _ => "en_craft_80.png",
    }
}

impl View for SkillBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((22, 22, 22)));
        canvas.fill_rect(SDLRect::new(
            self.position.x,
            self.position.y,
            ((ICON_SIZE + BORDER_WIDTH) * ICON_COUNT as i32 + BORDER_WIDTH) as u32,
            (ICON_SIZE + BORDER_WIDTH * 2) as u32,
        ))?;

        for view in self.views.iter() {
            view.render(ecs, canvas, frame)?;
        }

        Ok(())
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        for view in self.views.iter() {
            if view.rect.contains_point(SDLPoint::new(x, y)) {
                return view.hit_test(ecs, x, y);
            }
        }
        None
    }
}

pub struct SkillBarItemView {
    pub rect: SDLRect,
    index: u32,
    image: Texture,
    hotkey: ((u32, u32), Texture),
}

impl SkillBarItemView {
    pub fn init(position: SDLPoint, index: u32, image: Texture, hotkey: ((u32, u32), Texture)) -> BoxResult<SkillBarItemView> {
        let rect = SDLRect::new(position.x, position.y, 44, 44);
        Ok(SkillBarItemView { rect, index, image, hotkey })
    }
}

impl View for SkillBarItemView {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.copy(&self.image, SDLRect::new(0, 0, 256, 256), self.rect)?;
        let ((width, height), texture) = &self.hotkey;
        let hotkey_bounds = SDLRect::new(2 + self.rect.x() as i32, 24 + self.rect.y() as i32, *width, *height);
        let hotkey_background_bounds = SDLRect::new(hotkey_bounds.x() - 2, hotkey_bounds.y(), hotkey_bounds.width() + 4, hotkey_bounds.height());
        canvas.set_draw_color(Color::RGBA(32, 32, 32, 200));
        canvas.fill_rect(hotkey_background_bounds)?;
        canvas.copy(&texture, SDLRect::new(0, 0, *width, *height), hotkey_bounds)?;
        Ok(())
    }

    fn hit_test(&self, _: &World, _: i32, _: i32) -> Option<HitTestResult> {
        Some(HitTestResult::Skill(test_skill_name(self.index).to_string()))
    }
}
