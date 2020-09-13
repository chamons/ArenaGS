use std::time::Duration;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

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

            let direction = self.scene.ask_stage_direction();

            match self.storyteller.follow_stage_direction(direction, &render_context) {
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

    pub fn screen_shot(render_context: &RenderContextHolder) -> BoxResult<Texture> {
        let render = render_context.borrow_mut();
        let output_size = render.canvas.output_size()?;
        let mut pixels = render
            .canvas
            .read_pixels(SDLRect::new(0, 0, output_size.0, output_size.1), PixelFormatEnum::ARGB8888)?;
        let pitch = output_size.0 * 4;
        let screen = sdl2::surface::Surface::from_data(&mut pixels, output_size.0, output_size.1, pitch, PixelFormatEnum::ARGB8888)?;
        let texture_creator = render.canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&screen).map_err(|e| e.to_string())?;

        Ok(texture)
    }
}
