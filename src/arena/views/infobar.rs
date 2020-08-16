use std::rc::Rc;

use enum_iterator::IntoEnumIterator;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::View;
use crate::after_image::{FontColor, FontSize, RenderCanvas, TextRenderer};
use crate::atlas::{BoxResult, EasyECS};
use crate::clash::{find_player, AmmoKind, SkillResourceComponent};

pub struct InfoBarView {
    position: SDLPoint,
    text: Rc<TextRenderer>,
}

impl InfoBarView {
    pub fn init(position: SDLPoint, text: Rc<TextRenderer>) -> BoxResult<InfoBarView> {
        Ok(InfoBarView { position, text })
    }
    fn render_character_info(&self, ecs: &World, canvas: &mut RenderCanvas) -> BoxResult<()> {
        self.text
            .render_text("Info Bar", self.position.x, self.position.y, canvas, FontSize::Large, FontColor::Black)?;

        let resources = &ecs.read_storage::<SkillResourceComponent>();
        let resource = resources.grab(find_player(&ecs));
        self.text.render_text(
            format!("Exhaustion: {:.2}", resource.exhaustion).as_str(),
            self.position.x + 4,
            self.position.y + 30,
            canvas,
            FontSize::Small,
            FontColor::Black,
        )?;

        self.text.render_text(
            format!("Focus: {:.2}", resource.focus).as_str(),
            self.position.x + 4,
            self.position.y + 50,
            canvas,
            FontSize::Small,
            FontColor::Black,
        )?;

        for kind in AmmoKind::into_enum_iter() {
            match resource.max.get(&kind) {
                Some(value) => {
                    self.text.render_text(
                        format!("{:?}: {:.2}/{:.2}", kind, resource.ammo[&kind], value).as_str(),
                        self.position.x + 4,
                        self.position.y + 70,
                        canvas,
                        FontSize::Small,
                        FontColor::Black,
                    )?;
                }
                None => {}
            }
        }

        Ok(())
    }
}

impl View for InfoBarView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((196, 196, 0)));
        canvas.fill_rect(SDLRect::new(self.position.x, self.position.y, 230, 400))?;
        self.render_character_info(ecs, canvas)?;

        Ok(())
    }
}
