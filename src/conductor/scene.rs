use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::{MouseButton, MouseState};

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::conductor::StageDirection;

pub trait Scene {
    fn handle_mouse_click(&mut self, x: i32, y: i32, button: Option<MouseButton>) {}

    fn handle_mouse_move(&mut self, x: i32, y: i32, state: MouseState) {}

    fn handle_key(&mut self, keycode: Keycode, keymod: Mod) {}

    fn render(&mut self, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()>;

    fn tick(&mut self, _frame: u64);

    fn on_quit(&mut self) -> BoxResult<()>;

    fn ask_stage_direction(&self) -> StageDirection;
}
