use std::rc::Rc;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{wrap_progression, ProgressionState};
use crate::conductor::{Scene, StageDirection};

pub struct RewardScene {
    interacted: bool,
    progression: ProgressionState,
    text_renderer: Rc<TextRenderer>,
}

impl RewardScene {
    pub fn init(_render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>, progression: ProgressionState) -> BoxResult<RewardScene> {
        Ok(RewardScene {
            interacted: false,
            progression,
            text_renderer: Rc::clone(text_renderer),
        })
    }
}

impl Scene for RewardScene {
    fn handle_key(&mut self, _keycode: Keycode, _keymod: Mod) {}

    fn handle_mouse(&mut self, _x: i32, _y: i32, button: Option<MouseButton>) {
        if button.is_some() {
            self.interacted = true;
        }
    }

    fn render(&mut self, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();

        self.text_renderer.render_text("Reward", 50, 50, canvas, FontSize::Large, FontColor::White)?;

        canvas.present();

        Ok(())
    }

    fn tick(&mut self, _frame: u64) {}

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&self) -> StageDirection {
        if self.interacted {
            StageDirection::ShowCharacter(wrap_progression(&self.progression))
        } else {
            StageDirection::Continue
        }
    }
}
