use std::rc::Rc;

use enum_iterator::IntoEnumIterator;
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::view_components::{Frame, FrameKind};
use super::View;
use crate::after_image::{FontColor, FontSize, IconLoader, RenderCanvas, RenderContext, TextRenderer};
use crate::atlas::{BoxResult, EasyECS};
use crate::clash::{find_enemies, find_player, summarize_character, AmmoKind, CharacterInfoComponent, SkillResourceComponent, StatusComponent};

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
        summarize_character(ecs, entity, show_status_effect, |t| self.small_text(canvas, t, offset).unwrap());
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
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        self.frame.render(ecs, canvas, frame)?;
        self.render_character_info(ecs, canvas)?;

        Ok(())
    }
}
