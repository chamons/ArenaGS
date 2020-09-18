use std::rc::Rc;

use enum_iterator::IntoEnumIterator;
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::view_components::{Frame, FrameKind};
use super::{ContextData, View};
use crate::after_image::{FontColor, FontSize, IconLoader, RenderCanvas, RenderContext, TextRenderer};
use crate::atlas::{BoxResult, EasyECS};
use crate::clash::{find_enemies, find_player, AmmoKind, CharacterInfoComponent, SkillResourceComponent, StatusComponent};

pub struct InfoBarView {
    position: SDLPoint,
    text: Rc<TextRenderer>,
    frame: Frame,
}

impl InfoBarView {
    pub fn init(position: SDLPoint, render_context: &RenderContext, text: Rc<TextRenderer>) -> BoxResult<InfoBarView> {
        Ok(InfoBarView {
            position,
            text,
            frame: Frame::init(
                SDLPoint::new(position.x() - 27, position.y() - 20),
                render_context,
                &IconLoader::init_ui()?,
                FrameKind::InfoBar,
            )?,
        })
    }

    fn render_character_info(&self, ecs: &World, canvas: &mut RenderCanvas) -> BoxResult<()> {
        let mut offset = 5;
        self.render_character(canvas, ecs, find_player(&ecs), &mut offset, false)?;
        offset += 40;

        for e in find_enemies(&ecs) {
            self.small_text(canvas, "Enemy:", &mut offset)?;
            self.render_character(canvas, ecs, e, &mut offset, true)?;
            offset += 20;
        }
        Ok(())
    }

    fn render_character(&self, canvas: &mut RenderCanvas, ecs: &World, entity: Entity, offset: &mut i32, show_status_effect: bool) -> BoxResult<()> {
        let char_infos = &ecs.read_storage::<CharacterInfoComponent>();
        let char_info = char_infos.grab(entity);
        let defenses = &char_info.character.defenses;
        let health_text = {
            if defenses.absorb != 0 {
                format!("Health: (+{:.2}) {:.2}/{:.2}", defenses.absorb, defenses.health, defenses.max_health)
            } else {
                format!("Health: {:.2}/{:.2}", defenses.health, defenses.max_health)
            }
        };
        self.small_text(canvas, health_text.as_str(), offset)?;

        if defenses.max_dodge != 0 {
            self.small_text(canvas, format!("Dodge : {:.2}/{:.2}", defenses.dodge, defenses.max_dodge).as_str(), offset)?;
        }
        if defenses.armor != 0 {
            self.small_text(canvas, format!("Armor: {:.2}", defenses.armor).as_str(), offset)?;
        }

        let resources = &ecs.read_storage::<SkillResourceComponent>();
        if let Some(resource) = resources.get(entity) {
            self.small_text(canvas, format!("Exhaustion: {:.2}", resource.exhaustion).as_str(), offset)?;

            self.small_text(canvas, format!("Focus: {:.2}", resource.focus).as_str(), offset)?;

            for kind in AmmoKind::into_enum_iter() {
                match resource.max.get(&kind) {
                    Some(value) => {
                        self.small_text(canvas, format!("{:?}: {:.2}/{:.2}", kind, resource.ammo[&kind], value).as_str(), offset)?;
                    }
                    None => {}
                }
            }
        }

        let temperature = char_info.character.temperature.current_temperature();
        if temperature != 0 {
            self.small_text(canvas, format!("Temperature: {:.2}", temperature).as_str(), offset)?;
        }

        if show_status_effect {
            let statuses = &ecs.read_storage::<StatusComponent>();
            if let Some(status) = statuses.get(entity) {
                let all = status.status.get_all_status();
                if !all.is_empty() {
                    self.small_text(canvas, &format!("Status: {}", status.status.get_all_status().join(" ")), offset)?;
                }
            }
        }
        Ok(())
    }

    const MAX_INFO_OFFSET: i32 = 480;
    fn small_text(&self, canvas: &mut RenderCanvas, text: &str, offset: &mut i32) -> BoxResult<()> {
        if *offset > InfoBarView::MAX_INFO_OFFSET {
            return Ok(());
        }
        self.text
            .render_text(text, self.position.x + 4, self.position.y + *offset, canvas, FontSize::Small, FontColor::Black)?;
        *offset += 20;
        Ok(())
    }
}

impl View for InfoBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64, context: &ContextData) -> BoxResult<()> {
        self.frame.render(ecs, canvas, frame, context)?;
        self.render_character_info(ecs, canvas)?;

        Ok(())
    }
}
