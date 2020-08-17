use std::rc::Rc;

use enum_iterator::IntoEnumIterator;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::View;
use crate::after_image::{FontColor, FontSize, RenderCanvas, TextRenderer};
use crate::atlas::{BoxResult, EasyECS};
use crate::clash::{find_player, AmmoKind, CharacterInfoComponent, SkillResourceComponent};

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

        let mut offset = 30;
        let player = find_player(&ecs);
        let char_infos = &ecs.read_storage::<CharacterInfoComponent>();
        let char_info = char_infos.grab(player);

        // Write out Dodge,Current Dodge, Armor, Absorb, and Health (if not zero)
        let temperature = char_info.character.temperature.current_temperature;
        if temperature != 0 {
            self.text.render_text(
                format!("Temperature: {:.2}", temperature).as_str(),
                self.position.x + 4,
                self.position.y + offset,
                canvas,
                FontSize::Small,
                FontColor::Black,
            )?;
            offset += 20;
        }

        let resources = &ecs.read_storage::<SkillResourceComponent>();
        let resource = resources.grab(player);
        self.text.render_text(
            format!("Exhaustion: {:.2}", resource.exhaustion).as_str(),
            self.position.x + 4,
            self.position.y + offset,
            canvas,
            FontSize::Small,
            FontColor::Black,
        )?;
        offset += 20;

        self.text.render_text(
            format!("Focus: {:.2}", resource.focus).as_str(),
            self.position.x + 4,
            self.position.y + offset,
            canvas,
            FontSize::Small,
            FontColor::Black,
        )?;
        offset += 20;

        for kind in AmmoKind::into_enum_iter() {
            match resource.max.get(&kind) {
                Some(value) => {
                    self.text.render_text(
                        format!("{:?}: {:.2}/{:.2}", kind, resource.ammo[&kind], value).as_str(),
                        self.position.x + 4,
                        self.position.y + offset,
                        canvas,
                        FontSize::Small,
                        FontColor::Black,
                    )?;
                    offset += 20;
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
