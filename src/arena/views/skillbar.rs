use std::cmp;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::super::{battle_actions, IconLoader};
use super::{ContextData, HitTestResult, View};
use crate::after_image::{FontColor, FontSize, RenderCanvas, RenderContext, RenderContextHolder, TextRenderer};
use crate::atlas::{BoxResult, EasyECS};
use crate::clash::{find_player, get_skill, SkillsComponent};

pub struct SkillBarView {
    position: SDLPoint,
    views: Vec<SkillBarItemView>,
}

const BORDER_WIDTH: i32 = 5;
const ICON_SIZE: i32 = 44;
const MAX_ICON_COUNT: i32 = 10;

impl SkillBarView {
    pub fn init(render_context_holder: &RenderContextHolder, ecs: &World, position: SDLPoint, text: Rc<TextRenderer>) -> BoxResult<SkillBarView> {
        let mut views = Vec::with_capacity(15);
        let icons = IconLoader::init(Rc::clone(render_context_holder), "spell")?;
        for i in 0..get_skill_count(ecs) {
            if let Some(skill_name) = battle_actions::get_skill_name(ecs, i as usize) {
                let render_context = &render_context_holder.borrow();
                let view = SkillBarItemView::init(
                    SDLPoint::new(
                        get_skillbar_offset(ecs, position) + BORDER_WIDTH + (ICON_SIZE + BORDER_WIDTH) * i as i32,
                        position.y + BORDER_WIDTH + 1,
                    ),
                    render_context,
                    Rc::clone(&text),
                    &icons,
                    i as u32,
                    skill_name.as_str(),
                )?;
                views.push(view);
            }
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
        canvas.set_draw_color(Color::from((22, 22, 22)));

        let skill_count = get_skill_count(ecs);
        canvas.fill_rect(SDLRect::new(
            get_skillbar_offset(ecs, self.position),
            self.position.y,
            ((ICON_SIZE + BORDER_WIDTH) * skill_count as i32 + BORDER_WIDTH) as u32,
            (ICON_SIZE + BORDER_WIDTH * 2) as u32,
        ))?;

        for view in self.views.iter() {
            view.render(ecs, canvas, frame, context)?;
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
    skill_name: String,
    hotkey: ((u32, u32), Texture),
    hotkey_inactive: ((u32, u32), Texture),
    alternate_image: Option<Texture>,
}
type HotKeyRenderInfo = ((u32, u32), Texture);

impl SkillBarItemView {
    pub fn init(
        position: SDLPoint,
        render_context: &RenderContext,
        text: Rc<TextRenderer>,
        icons: &IconLoader,
        index: u32,
        skill_name: &str,
    ) -> BoxResult<SkillBarItemView> {
        let rect = SDLRect::new(position.x, position.y, 44, 44);
        let skill = get_skill(&skill_name);
        let image = icons.get(skill.image)?;
        let hotkey = text.render_texture(&render_context.canvas, &index.to_string(), FontSize::Bold, FontColor::White)?;
        let hotkey_inactive = text.render_texture(&render_context.canvas, &index.to_string(), FontSize::Bold, FontColor::Red)?;
        let alternate_image = match &skill.alternate {
            Some(alternate) => Some(icons.get(get_skill(alternate).image)?),
            None => None,
        };

        Ok(SkillBarItemView {
            rect,
            index,
            image,
            skill_name: skill_name.to_string(),
            hotkey,
            hotkey_inactive,
            alternate_image,
        })
    }

    fn get_render_params(&self, ecs: &World) -> (&HotKeyRenderInfo, &Texture, bool) {
        let skill = get_skill(&self.skill_name);

        if skill.is_usable(ecs, &find_player(&ecs)) {
            (&self.hotkey, &self.image, false)
        } else if skill.alternate.is_some() {
            (&self.hotkey, &self.alternate_image.as_ref().unwrap(), false)
        } else {
            (&self.hotkey_inactive, &self.image, true)
        }
    }
}

impl View for SkillBarItemView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64, _context: &ContextData) -> BoxResult<()> {
        let (((width, height), hotkey_texture), texture, disable_overlay) = self.get_render_params(ecs);

        canvas.copy(texture, SDLRect::new(0, 0, 256, 256), self.rect)?;
        let hotkey_bounds = SDLRect::new(2 + self.rect.x() as i32, 24 + self.rect.y() as i32, *width, *height);
        let hotkey_background_bounds = SDLRect::new(hotkey_bounds.x() - 2, hotkey_bounds.y(), hotkey_bounds.width() + 4, hotkey_bounds.height());
        canvas.set_draw_color(Color::RGBA(32, 32, 32, 200));
        canvas.fill_rect(hotkey_background_bounds)?;
        canvas.copy(&hotkey_texture, SDLRect::new(0, 0, *width, *height), hotkey_bounds)?;

        if disable_overlay {
            canvas.set_draw_color(Color::RGBA(12, 12, 12, 196));
            canvas.fill_rect(self.rect)?;
        }

        Ok(())
    }

    fn hit_test(&self, ecs: &World, _: i32, _: i32) -> Option<HitTestResult> {
        if battle_actions::get_skill_name(ecs, self.index as usize).is_some() {
            Some(HitTestResult::Skill(battle_actions::get_current_skill(ecs, &self.skill_name)))
        } else {
            None
        }
    }
}
