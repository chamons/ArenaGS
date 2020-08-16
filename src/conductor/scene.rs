use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use super::EventStatus;
use crate::after_image::RenderCanvas;
use crate::atlas::BoxResult;

pub trait Scene {
    fn handle_mouse(&mut self, x: i32, y: i32, button: Option<MouseButton>) -> EventStatus;
    fn handle_key(&mut self, keycode: Keycode) -> EventStatus;

    fn render(&self, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.clear();
        Ok(())
    }

    fn tick(&mut self, _frame: u64) -> BoxResult<EventStatus> {
        Ok(EventStatus::Continue)
    }

    fn on_quit(&mut self) -> BoxResult<()>;
}
