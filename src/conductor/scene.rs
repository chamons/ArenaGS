use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use super::EventStatus;
use crate::after_image::BoxResult;

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

    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> BoxResult<()> {
        canvas.clear();
        Ok(())
    }
}
