use specs::prelude::*;

use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::after_image::RenderCanvas;
use crate::atlas::BoxResult;

pub trait Scene {
    fn handle_mouse(&mut self, x: i32, y: i32, button: Option<MouseButton>);
    fn handle_key(&mut self, keycode: Keycode);

    fn render(&mut self, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()>;

    fn tick(&mut self, _frame: u64);

    fn on_quit(&mut self) -> BoxResult<()>;
    fn get_state(&self) -> &World;
}
