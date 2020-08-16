use std::time::Duration;
use std::time::Instant;

use sdl2::event::Event;

use super::Scene;

use crate::after_image::RenderContextHolder;
use crate::atlas::BoxResult;

#[allow(dead_code)]
pub enum EventStatus {
    Quit,
    Continue,
    NewScene(Box<dyn Scene>),
}

pub struct Director<'a> {
    scene: Box<dyn Scene + 'a>,
}

impl<'a> Director<'a> {
    pub fn init(scene: Box<dyn Scene + 'a>) -> Director {
        Director { scene }
    }

    pub fn change_scene(&mut self, scene: Box<dyn Scene + 'a>) {
        self.scene = scene;
    }
    pub fn run(&mut self, render_context: RenderContextHolder) -> BoxResult<()> {
        let mut frame = 0;
        loop {
            let start_frame = Instant::now();
            for event in render_context.borrow_mut().event_pump.poll_iter() {
                let status = match event {
                    Event::Quit { .. } => {
                        self.scene.on_quit()?;
                        EventStatus::Quit
                    }
                    Event::KeyDown { keycode, repeat: false, .. } => {
                        if let Some(keycode) = keycode {
                            self.scene.handle_key(keycode)
                        } else {
                            EventStatus::Continue
                        }
                    }
                    Event::MouseButtonDown { x, y, mouse_btn, .. } => self.scene.handle_mouse(x, y, Some(mouse_btn)),
                    Event::MouseMotion { x, y, .. } => self.scene.handle_mouse(x, y, None),
                    _ => EventStatus::Continue,
                };

                match status {
                    EventStatus::Quit => return Ok(()),
                    EventStatus::NewScene(s) => self.change_scene(s),
                    EventStatus::Continue => {}
                }
            }

            self.scene.tick(frame)?;

            self.scene.render(&mut render_context.borrow_mut().canvas, frame)?;

            let end_frame = Instant::now();
            if let Some(duration) = end_frame.checked_duration_since(start_frame) {
                let ms = duration.as_millis() as u64;
                if ms < 16 {
                    ::std::thread::sleep(Duration::from_millis(16 - ms));
                }
            }

            frame += 1;
        }
    }
}
