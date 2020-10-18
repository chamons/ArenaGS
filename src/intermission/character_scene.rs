use std::rc::Rc;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use specs::prelude::*;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::conductor::{Scene, StageDirection};

pub struct CharacterScene {
    interacted: bool,
    phase: u32,
    text_renderer: Rc<TextRenderer>,
}

impl CharacterScene {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>, phase: u32) -> BoxResult<CharacterScene> {
        Ok(CharacterScene {
            interacted: false,
            phase,
            text_renderer: Rc::clone(text_renderer),
        })
    }
}

impl Scene for CharacterScene {
    fn handle_key(&mut self, keycode: Keycode, _keymod: Mod) {}

    fn handle_mouse(&mut self, x: i32, y: i32, button: Option<MouseButton>) {
        if let Some(button) = button {
            self.interacted = true;
        }
    }

    fn render(&mut self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();

        self.text_renderer.render_text("Character", 50, 50, canvas, FontSize::Large, FontColor::White)?;

        canvas.present();

        Ok(())
    }

    fn tick(&mut self, frame: u64) {}

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&self) -> StageDirection {
        if self.interacted {
            StageDirection::NewRound(self.phase)
        } else {
            StageDirection::Continue
        }
    }
}
