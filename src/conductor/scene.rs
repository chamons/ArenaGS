use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use super::EventStatus;
use crate::after_image::RenderCanvas;
use crate::atlas::BoxResult;

pub trait Scene {
    fn handle_event(&self, event: &sdl2::event::Event) -> EventStatus {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return EventStatus::Quit,
            _ => {}
        }
        EventStatus::Continue
    }

    fn render(&self, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.clear();
        Ok(())
    }

    fn tick(&mut self, _frame: u64) -> BoxResult<()> {
        Ok(())
    }
}
