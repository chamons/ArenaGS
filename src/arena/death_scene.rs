use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::{BlendMode, Texture};

use crate::after_image::RenderCanvas;
use crate::atlas::BoxResult;
use crate::conductor::{Scene, StageDirection};

pub struct DeathScene {
    background: Texture,
    presentation_frame: u64,
}

impl DeathScene {
    pub fn init(background: Texture, message: String) -> DeathScene {
        DeathScene {
            background,
            presentation_frame: std::u64::MAX,
        }
    }

    fn get_frame_alpha(&mut self, frame: u64) -> u8 {
        self.presentation_frame = std::cmp::min(self.presentation_frame, frame);
        let frame = (frame - self.presentation_frame + 10) * 10;
        if frame > 200 {
            55u8
        } else {
            255u8 - frame as u8
        }
    }
}

impl Scene for DeathScene {
    fn handle_key(&mut self, keycode: Keycode) {}

    fn handle_mouse(&mut self, x: i32, y: i32, button: Option<MouseButton>) {}

    fn render(&mut self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        let alpha = self.get_frame_alpha(frame);
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

    fn tick(&mut self, frame: u64) {}

    fn on_quit(&mut self) -> BoxResult<()> {
        Ok(())
    }

    fn ask_stage_direction(&self) -> StageDirection {
        StageDirection::Continue
    }
}
