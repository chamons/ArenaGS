use std::time::Duration;
use std::time::Instant;

use sdl2::event::Event;

use super::{Scene, Storyteller};

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
    storyteller: Box<dyn Storyteller + 'a>,
}

impl<'a> Director<'a> {
    pub fn init(storyteller: Box<dyn Storyteller + 'a>) -> Director<'a> {
        let scene = storyteller.initial_scene();
        Director { scene, storyteller }
    }

    pub fn change_scene(&mut self, scene: Box<dyn Scene + 'a>) {
        self.scene = scene;
    }

    pub fn run(&mut self, render_context: RenderContextHolder) -> BoxResult<()> {
        let mut frame = 0;
        loop {
            let start_frame = Instant::now();
            for event in render_context.borrow_mut().event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        self.scene.on_quit()?;
                        return Ok(());
                    }
                    Event::KeyDown { keycode, repeat: false, .. } => {
                        if let Some(keycode) = keycode {
                            self.scene.handle_key(keycode)
                        }
                    }
                    Event::MouseButtonDown { x, y, mouse_btn, .. } => self.scene.handle_mouse(x, y, Some(mouse_btn)),
                    Event::MouseMotion { x, y, .. } => self.scene.handle_mouse(x, y, None),
                    _ => {}
                };
            }

            self.scene.tick(frame);

            match self.storyteller.check_place(self.scene.get_state()) {
                EventStatus::NewScene(scene) => self.change_scene(scene),
                EventStatus::Quit => return Ok(()),
                EventStatus::Continue => {}
            }

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
