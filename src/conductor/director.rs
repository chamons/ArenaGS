use std::time::Duration;
use std::time::Instant;

use super::Scene;
use crate::after_image::{BoxResult, RenderContext};

#[allow(dead_code)]
pub enum EventStatus {
    Quit,
    Continue,
    NewScene(Box<dyn Scene>),
}

pub struct Director {
    scene: Box<dyn Scene>,
}

impl Director {
    pub fn init(scene: Box<dyn Scene>) -> Director {
        Director { scene }
    }

    pub fn change_scene(&mut self, scene: Box<dyn Scene>) {
        self.scene = scene;
    }

    pub fn run(&mut self, render_context: &mut RenderContext) -> BoxResult<()> {
        loop {
            let start_frame = Instant::now();
            for event in render_context.event_pump.poll_iter() {
                match self.scene.handle_event(&event) {
                    EventStatus::Quit => return Ok(()),
                    EventStatus::NewScene(s) => self.change_scene(s),
                    EventStatus::Continue => {}
                }
            }

            self.scene.render(&mut render_context.canvas)?;

            let end_frame = Instant::now();
            if let Some(duration) = end_frame.checked_duration_since(start_frame) {
                let ms = duration.as_millis() as u64;
                ::std::thread::sleep(Duration::from_millis(16 - ms));
            }
        }
    }
}
