use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::View;
use crate::after_image::{FontColor, FontSize, RenderCanvas, TextRenderer};
use crate::atlas::{BoxResult, EasyECS};
use crate::clash::{find_player, SkillResourceComponent};

pub struct InfoBarView<'a> {
    position: SDLPoint,
    text: &'a TextRenderer<'a>,
}

impl<'a> InfoBarView<'a> {
    pub fn init(position: SDLPoint, text: &'a TextRenderer<'a>) -> BoxResult<InfoBarView> {
        Ok(InfoBarView { position, text })
    }
    fn render_character_info(&self, ecs: &World, canvas: &mut RenderCanvas) -> BoxResult<()> {
        self.text
            .render_text("Info Bar", self.position.x, self.position.y, canvas, FontSize::Large, FontColor::Black)?;

        let resources = &ecs.read_storage::<SkillResourceComponent>();
        let resource = resources.grab(find_player(&ecs));
        self.text.render_text(
            format!("Exhaustion: {}", resource.exhaustion).as_str(),
            self.position.x,
            self.position.y + 30,
            canvas,
            FontSize::Small,
            FontColor::Black,
        )?;

        self.text.render_text(
            format!("Focus: {}", resource.focus).as_str(),
            self.position.x,
            self.position.y + 50,
            canvas,
            FontSize::Small,
            FontColor::Black,
        )?;

        Ok(())
    }
}

impl<'a> View for InfoBarView<'a> {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((196, 196, 0)));
        canvas.fill_rect(SDLRect::new(self.position.x, self.position.y, 230, 400))?;
        self.render_character_info(ecs, canvas)?;

        Ok(())
    }
}
