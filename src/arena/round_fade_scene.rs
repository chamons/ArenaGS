use std::cmp;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{wrap_progression, ProgressionState};
use crate::conductor::{Scene, StageDirection};

pub struct RoundFadeScene {
    background: Texture,
    presentation_frame: u64,
    interacted: bool,
    progression: ProgressionState,
}

impl RoundFadeScene {
    pub fn init(background: Texture, progression: ProgressionState) -> RoundFadeScene {
        RoundFadeScene {
            background,
            presentation_frame: std::u64::MAX,
            interacted: false,
            progression,
        }
    }

    fn get_frame_alpha(&mut self, frame: u64) -> u8 {
        self.presentation_frame = cmp::min(self.presentation_frame, frame);
        let frame = (frame - self.presentation_frame + 10) * 10;
        if frame > 200 {
            25u8
        } else {
            255u8 - frame as u8
        }
    }
}

impl Scene for RoundFadeScene {
    fn handle_key(&mut self, _keycode: Keycode, _keymod: Mod) {
        self.interacted = true;
    }

    fn handle_mouse_click(&mut self, _x: i32, _y: i32, button: Option<MouseButton>) {
        if button.is_some() {
            self.interacted = true;
        }
    }

    fn render(&mut self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        let alpha = self.get_frame_alpha(frame);
        if alpha == 25 {
            self.interacted = true;
        }

        let output_size = canvas.output_size()?;

        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();

        self.background.set_alpha_mod(alpha);

        canvas.copy(
            &self.background,
            SDLRect::new(0, 0, output_size.0, output_size.1),
            SDLRect::new(0, 0, output_size.0, output_size.1),
        )?;

        canvas.present();

        Ok(())
    }

    fn tick(&mut self, _frame: u64) {}

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&self) -> StageDirection {
        if self.interacted {
            StageDirection::ShowRewards(wrap_progression(&self.progression))
        } else {
            StageDirection::Continue
        }
    }
}
