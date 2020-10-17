use std::rc::Rc;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use specs::prelude::*;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::conductor::{Scene, StageDirection};

pub struct RewardScene {
    interacted: bool,
    phase: u32,
}

impl RewardScene {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>, phase: u32) -> BoxResult<RewardScene> {
        Ok(RewardScene { interacted: false, phase })
    }
}

impl Scene for RewardScene {
    fn handle_key(&mut self, keycode: Keycode, _keymod: Mod) {}

    fn handle_mouse(&mut self, x: i32, y: i32, button: Option<MouseButton>) {}

    fn render(&mut self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        self.interacted = true;
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
