use std::cmp;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::super::battle_actions;
use super::{ContextData, HitTestResult, View};
use crate::after_image::{FontColor, FontSize, IconCache, IconLoader, RenderCanvas, RenderContext, TextRenderer};
use crate::atlas::{BoxResult, EasyECS};
use crate::clash::{all_skill_image_filesnames, find_player, get_skill, SkillsComponent};

pub struct SkillBarView {
    position: SDLPoint,
    views: Vec<SkillBarItemView>,
}

const BORDER_WIDTH: i32 = 4;
const ICON_SIZE: i32 = 48;
const MAX_ICON_COUNT: i32 = 10;

impl SkillBarView {
    pub fn init(render_context: &RenderContext, ecs: &World, position: SDLPoint, text: Rc<TextRenderer>) -> BoxResult<SkillBarView> {
        let mut views = Vec::with_capacity(10);
        let cache = Rc::new(IconCache::init(render_context, IconLoader::init()?, &all_skill_image_filesnames())?);

        for i in 0..get_skill_count(ecs) {
            let position = SDLPoint::new(
                get_skillbar_offset(ecs, position) + BORDER_WIDTH + (ICON_SIZE + BORDER_WIDTH) * i as i32,
                position.y + BORDER_WIDTH + 1,
            );

            let view = SkillBarItemView::init(position, render_context, Rc::clone(&text), i, &cache)?;
            views.push(view);
        }
        Ok(SkillBarView { position, views })
    }
}

fn get_skillbar_offset(ecs: &World, position: SDLPoint) -> i32 {
    let skill_count = get_skill_count(ecs) as i32;
    position.x + ((MAX_ICON_COUNT - skill_count) * (ICON_SIZE + BORDER_WIDTH) / 2)
}

fn get_skill_count(ecs: &World) -> usize {
    let skills = ecs.read_storage::<SkillsComponent>();
    let player = find_player(&ecs);
    cmp::min(skills.grab(player).skills.len(), MAX_ICON_COUNT as usize)
}

impl View for SkillBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64, context: &ContextData) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((33, 33, 33)));

        let skill_count = get_skill_count(ecs);
        if skill_count > 0 {
            canvas.fill_rect(SDLRect::new(
                get_skillbar_offset(ecs, self.position) - (BORDER_WIDTH / 2),
                self.position.y,
                ((ICON_SIZE + BORDER_WIDTH) * skill_count as i32 + BORDER_WIDTH) as u32,
                (ICON_SIZE + BORDER_WIDTH * 2) as u32,
            ))?;

            for view in self.views.iter() {
                view.render(ecs, canvas, frame, context)?;
            }
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
    index: usize,
    icons: Rc<IconCache>,
    hotkey: ((u32, u32), Texture),
    hotkey_inactive: ((u32, u32), Texture),
}
type HotKeyRenderInfo = ((u32, u32), Texture);

impl SkillBarItemView {
    pub fn init(
        position: SDLPoint,
        render_context: &RenderContext,
        text: Rc<TextRenderer>,
        index: usize,
        icons: &Rc<IconCache>,
    ) -> BoxResult<SkillBarItemView> {
        let rect = SDLRect::new(position.x, position.y, 44, 44);

        let hotkey_number = skill_index_to_hotkey(index);
        let hotkey = text.render_texture(&render_context.canvas, &hotkey_number.to_string(), FontSize::Bold, FontColor::White)?;
        let hotkey_inactive = text.render_texture(&render_context.canvas, &hotkey_number.to_string(), FontSize::Bold, FontColor::Red)?;

        Ok(SkillBarItemView {
            rect,
            index,
            icons: Rc::clone(icons),
            hotkey,
            hotkey_inactive,
        })
    }

    fn get_render_params(&self, ecs: &World) -> Option<(&HotKeyRenderInfo, &Texture, bool)> {
        if let Some(skill_name) = battle_actions::get_skill_name(ecs, self.index) {
            let skill = get_skill(&skill_name);

            if skill.is_usable(ecs, &find_player(&ecs)) {
                Some((&self.hotkey, self.icons.get(&skill.image.unwrap()), false))
            } else if let Some(alt_skill_name) = &skill.alternate {
                let alternate_skill = get_skill(&alt_skill_name);
                Some((&self.hotkey, self.icons.get(&alternate_skill.image.unwrap()), false))
            } else {
                Some((&self.hotkey_inactive, self.icons.get(&skill.image.unwrap()), true))
            }
        } else {
            None
        }
    }
}

impl View for SkillBarItemView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64, _context: &ContextData) -> BoxResult<()> {
        if let Some((((width, height), hotkey_texture), texture, disable_overlay)) = self.get_render_params(ecs) {
            canvas.copy(texture, SDLRect::new(0, 0, ICON_SIZE as u32, ICON_SIZE as u32), self.rect)?;
            let hotkey_bounds = SDLRect::new(2 + self.rect.x() as i32, 24 + self.rect.y() as i32, *width, *height);
            let hotkey_background_bounds = SDLRect::new(hotkey_bounds.x() - 2, hotkey_bounds.y(), hotkey_bounds.width() + 4, hotkey_bounds.height());
            canvas.set_draw_color(Color::RGBA(32, 32, 32, 200));
            canvas.fill_rect(hotkey_background_bounds)?;
            canvas.copy(&hotkey_texture, SDLRect::new(0, 0, *width, *height), hotkey_bounds)?;

            if disable_overlay {
                canvas.set_draw_color(Color::RGBA(12, 12, 12, 196));
                canvas.fill_rect(self.rect)?;
            }
        }

        Ok(())
    }

    fn hit_test(&self, ecs: &World, _: i32, _: i32) -> Option<HitTestResult> {
        if let Some(skill_name) = battle_actions::get_skill_name(ecs, self.index) {
            Some(HitTestResult::Skill(battle_actions::get_current_skill(ecs, &skill_name)))
        } else {
            None
        }
    }
}

fn skill_index_to_hotkey(index: usize) -> usize {
    assert!(index < 10);
    if index == 9 {
        0
    } else {
        index + 1
    }
}

pub fn hotkey_to_skill_index(hotkey: usize) -> usize {
    assert!(hotkey <= 10);
    if hotkey == 0 {
        9
    } else {
        hotkey - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skill_hotkey_mapping() {
        assert_eq!(1, skill_index_to_hotkey(0));
        assert_eq!(2, skill_index_to_hotkey(1));
        assert_eq!(9, skill_index_to_hotkey(8));
        assert_eq!(0, skill_index_to_hotkey(9));
    }

    #[test]
    #[should_panic]
    fn skill_hotkey_out_of_range() {
        skill_index_to_hotkey(10);
    }

    #[test]
    fn hotkey_to_skill_mapping() {
        assert_eq!(0, hotkey_to_skill_index(1));
        assert_eq!(6, hotkey_to_skill_index(7));
        assert_eq!(9, hotkey_to_skill_index(0));
    }

    #[test]
    fn hotkey_to_skill_out_of_range() {
        hotkey_to_skill_index(10);
    }
}
