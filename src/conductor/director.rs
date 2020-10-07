use std::time::Duration;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use sdl2::video::FullscreenType;

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

    fn is_alt(keymod: Mod) -> bool {
        keymod & Mod::LALTMOD == Mod::LALTMOD || keymod & Mod::RALTMOD == Mod::RALTMOD
    }

    fn is_ctrl(keymod: Mod) -> bool {
        keymod & Mod::LCTRLMOD == Mod::LCTRLMOD || keymod & Mod::RCTRLMOD == Mod::RCTRLMOD
    }

    pub fn run(&mut self, render_context: RenderContextHolder) -> BoxResult<()> {
        let mut frame = 0;
        loop {
            let start_frame = Instant::now();

            let mut change_fullscreen_state = false;
            {
                let render_context = &mut render_context.borrow_mut();

                // We only need this for mouse down (ctrl left clicking to bring up help)
                // but we are unable to access it there since we're inside event_pump pumping
                // and the borrow checker would slap us.
                let keymod = render_context.keyboard_util.mod_state();

                for event in render_context.event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => {
                            self.scene.on_quit()?;
                            return Ok(());
                        }
                        Event::KeyDown {
                            keycode,
                            repeat: false,
                            keymod,
                            ..
                        } => {
                            if let Some(keycode) = keycode {
                                match keycode {
                                    Keycode::Return => {
                                        if Director::is_alt(keymod) {
                                            change_fullscreen_state = true;
                                        }
                                    }
                                    _ => {}
                                }
                                self.scene.handle_key(keycode, keymod)
                            }
                        }
                        Event::MouseButtonDown { x, y, mut mouse_btn, .. } => {
                            if Director::is_ctrl(keymod) {
                                match mouse_btn {
                                    MouseButton::Left => {
                                        mouse_btn = MouseButton::Middle;
                                    }
                                    _ => {}
                                }
                            }
                            self.scene.handle_mouse(x, y, Some(mouse_btn));
                        }
                        Event::MouseMotion { x, y, .. } => self.scene.handle_mouse(x, y, None),
                        _ => {}
                    };
                }
            }
            if change_fullscreen_state {
                let mut render_context = render_context.borrow_mut();

                let (fullscreen_request, (width, height)) = match render_context.canvas.window().fullscreen_state() {
                    FullscreenType::Desktop | FullscreenType::True => (FullscreenType::Off, (1024, 768)),
                    FullscreenType::Off => (FullscreenType::True, (1366, 768)),
                };

                let window = render_context.canvas.window_mut();
                window.set_fullscreen(fullscreen_request)?;

                window.set_size(width, height).expect("Unable to set resolution on fullscreen swap");
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
