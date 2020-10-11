use std::cmp;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::{HitTestResult, View};
use crate::after_image::{FontColor, FontSize, IconCache, IconLoader, RenderCanvas, RenderContext, TextRenderer};
use crate::atlas::prelude::*;
use crate::clash::{all_skill_image_filesnames, find_player, get_skill, SkillsComponent, UsableResults};

pub struct SkillBarView {
    views: Vec<SkillBarItemView>,
}

const BORDER_WIDTH: i32 = 4;
const ICON_SIZE: i32 = 48;
const MAX_ICON_COUNT: i32 = 10;

impl SkillBarView {
    pub fn init(render_context: &RenderContext, ecs: &World, position: SDLPoint, text: Rc<TextRenderer>) -> BoxResult<SkillBarView> {
        let mut views = Vec::with_capacity(10);
        let cache = Rc::new(IconCache::init(render_context, IconLoader::init_icons(), &all_skill_image_filesnames())?);
        let ui = IconLoader::init_ui();

        for i in 0..get_skill_count(ecs) {
            let position = SDLPoint::new(
                get_skillbar_offset(ecs, position) + BORDER_WIDTH + (ICON_SIZE + BORDER_WIDTH) * i as i32,
                position.y + BORDER_WIDTH + 1,
            );

            let view = SkillBarItemView::init(
                position,
                render_context,
                Rc::clone(&text),
                i,
                &cache,
                ui.get(render_context, "skillbar_frame.png")?,
            )?;
            views.push(view);
        }
        Ok(SkillBarView { views })
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
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((33, 33, 33)));

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
    index: usize,
    icons: Rc<IconCache>,
    hotkey: ((u32, u32), Texture),
    hotkey_inactive: ((u32, u32), Texture),
    frame: Texture,
}
type HotKeyRenderInfo = ((u32, u32), Texture);

impl SkillBarItemView {
    pub fn init(
        position: SDLPoint,
        render_context: &RenderContext,
        text: Rc<TextRenderer>,
        index: usize,
        icons: &Rc<IconCache>,
        frame: Texture,
    ) -> BoxResult<SkillBarItemView> {
        let rect = SDLRect::new(position.x, position.y, 48, 48);

        let hotkey_number = skill_index_to_hotkey(index);
        let hotkey = text.render_texture(&render_context.canvas, &hotkey_number.to_string(), FontSize::Micro, FontColor::White)?;
        let hotkey_query = hotkey.query();

        let hotkey_inactive = text.render_texture(&render_context.canvas, &hotkey_number.to_string(), FontSize::Micro, FontColor::Red)?;
        let hotkey_inactive_query = hotkey.query();

        Ok(SkillBarItemView {
            rect,
            index,
            icons: Rc::clone(icons),
            hotkey: ((hotkey_query.width, hotkey_query.height), hotkey),
            hotkey_inactive: ((hotkey_inactive_query.width, hotkey_inactive_query.height), hotkey_inactive),
            frame,
        })
    }

    fn get_render_params(&self, ecs: &World) -> Option<(&HotKeyRenderInfo, &Texture, bool)> {
        if let Some(skill_name) = get_skill_name_on_skillbar(ecs, self.index) {
            let skill = get_skill(&skill_name);

            match skill.is_usable(ecs, find_player(&ecs)) {
                UsableResults::LacksAmmo if skill.alternate.is_some() => {
                    let alternate_skill = get_skill(skill.alternate.as_ref().unwrap());
                    Some((&self.hotkey, self.icons.get(&alternate_skill.image.unwrap()), false))
                }
                UsableResults::Usable => Some((&self.hotkey, self.icons.get(&skill.image.unwrap()), false)),
                _ => Some((&self.hotkey_inactive, self.icons.get(&skill.image.unwrap()), true)),
            }
        } else {
            None
        }
    }
}

impl View for SkillBarItemView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        if let Some((((width, height), hotkey_texture), texture, disable_overlay)) = self.get_render_params(ecs) {
            canvas.copy(texture, None, self.rect)?;
            if disable_overlay {
                canvas.set_draw_color(Color::RGBA(12, 12, 12, 196));
                canvas.fill_rect(self.rect)?;
            }

            canvas.copy(&self.frame, None, SDLRect::new(self.rect.x() - 2, self.rect.y() - 2, 52, 60))?;

            let hotkey_bounds = SDLRect::new(21 + self.rect.x() as i32, 43 + self.rect.y() as i32, *width, *height);
            canvas.copy(&hotkey_texture, SDLRect::new(0, 0, *width, *height), hotkey_bounds)?;
        }

        Ok(())
    }

    fn hit_test(&self, ecs: &World, _: i32, _: i32) -> Option<HitTestResult> {
        if let Some(skill_name) = get_skill_name_on_skillbar(ecs, self.index) {
            Some(HitTestResult::Skill(get_current_skill_on_skillbar(ecs, &skill_name)))
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

pub fn get_skill_name_on_skillbar(ecs: &World, index: usize) -> Option<String> {
    let skills_component = ecs.read_storage::<SkillsComponent>();
    let skills = &skills_component.grab(find_player(&ecs)).skills;
    skills.get(index).map(|s| get_current_skill_on_skillbar(ecs, s))
}

// Some skills have an alternate when not usable (such as reload)
pub fn get_current_skill_on_skillbar(ecs: &World, skill_name: &str) -> String {
    let skill = get_skill(skill_name);

    match skill.is_usable(ecs, find_player(&ecs)) {
        UsableResults::LacksAmmo if skill.alternate.is_some() => skill.alternate.as_ref().unwrap().to_string(),
        _ => skill_name.to_string(),
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
