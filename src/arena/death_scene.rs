use std::cmp;
use std::rc::Rc;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::ProgressionState;
use crate::conductor::{Scene, StageDirection};

pub struct DeathScene {
    screen_background: Texture,
    presentation_frame: u64,
    message: String,
    text: Rc<TextRenderer>,
    position: SDLRect,
    interacted: bool,
    frame: Texture,
}

impl DeathScene {
    pub fn init(screen_background: Texture, render_context: &RenderContextHolder, text: &Rc<TextRenderer>, message: String) -> BoxResult<DeathScene> {
        let output_size = { render_context.borrow().canvas.output_size()? };
        let (mid_x, mid_y) = ((output_size.0 / 2) as i32, (output_size.1 / 2) as i32);
        let box_width = 500;
        let box_height = 300;
        let position = SDLRect::from_center(SDLPoint::new(mid_x, mid_y), box_width, box_height);
        let ui = IconLoader::init_ui();

        // Default to interacted to skip dialog in self play
        let interacted = cfg!(feature = "self_play");

        Ok(DeathScene {
            screen_background,
            presentation_frame: std::u64::MAX,
            message,
            text: Rc::clone(text),
            position,
            interacted,
            frame: ui.get(&render_context.borrow(), "death_background.png")?,
        })
    }

    fn get_frame_alpha(&mut self, frame: u64) -> u8 {
        self.presentation_frame = cmp::min(self.presentation_frame, frame);
        let frame = (frame - self.presentation_frame + 10) * 10;
        if frame > 200 {
            55u8
        } else {
            255u8 - frame as u8
        }
    }

    fn small_text(&self, canvas: &mut RenderCanvas, text: &str, offset: &mut i32) -> BoxResult<()> {
        self.text.render_text(
            text,
            self.position.x() + 40,
            self.position.y() + 20 + *offset,
            canvas,
            FontSize::Small,
            FontColor::Black,
        )?;
        *offset += 20;
        Ok(())
    }
}

impl Scene for DeathScene {
    fn handle_key(&mut self, _keycode: Keycode, _keymod: Mod) {
        self.interacted = true;
    }

    fn handle_mouse(&mut self, _x: i32, _y: i32, button: Option<MouseButton>) {
        if button.is_some() {
            self.interacted = true;
        }
    }

    fn render(&mut self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        let alpha = self.get_frame_alpha(frame);
        let output_size = canvas.output_size()?;

        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();

        self.screen_background.set_alpha_mod(alpha);

        canvas.copy(
            &self.screen_background,
            SDLRect::new(0, 0, output_size.0, output_size.1),
            SDLRect::new(0, 0, output_size.0, output_size.1),
        )?;

        canvas.copy(&self.frame, None, self.position)?;

        let mut offset = 30;
        self.small_text(canvas, "You Died!", &mut offset)?;
        offset += 30;
        self.small_text(canvas, &self.message, &mut offset)?;

        canvas.present();

        Ok(())
    }

    fn tick(&mut self, _frame: u64) {}

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&self) -> StageDirection {
        if self.interacted {
            StageDirection::NewRound(super::arena_storyteller::create_stage_direction_from_state(&ProgressionState::init(0)))
        } else {
            StageDirection::Continue
        }
    }
}
