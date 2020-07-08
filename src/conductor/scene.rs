use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use super::EventStatus;
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

    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, _frame: u64) -> BoxResult<()> {
        canvas.clear();
        Ok(())
    }

    fn tick(&mut self) -> BoxResult<()> {
        Ok(())
    }
}
