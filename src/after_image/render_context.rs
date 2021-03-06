use std::{cell::RefCell, rc::Rc};

use super::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::image::{self, InitFlag};
use sdl2::rect::Rect as SDLRect;
use sdl2::render::BlendMode;

pub struct FontContext {
    pub ttf_context: sdl2::ttf::Sdl2TtfContext,
}

impl FontContext {
    pub fn initialize() -> Result<FontContext, String> {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        Ok(FontContext { ttf_context })
    }
}

pub type RenderContextHolder = Rc<RefCell<RenderContext>>;

pub struct RenderContext {
    _image_context: sdl2::image::Sdl2ImageContext,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pub keyboard_util: sdl2::keyboard::KeyboardUtil,
}

impl RenderContext {
    pub fn initialize() -> Result<RenderContext, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
        // stay unused because if we don't have any variable at all then Rust will treat it as a
        // temporary value and drop it right away!
        let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

        let window = video_subsystem
            .window("Arena GS", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .expect("Could not initialize video subsystem");

        let mut canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .accelerated()
            .build()
            .expect("Could not make a canvas");
        canvas.set_blend_mode(BlendMode::Blend);
        canvas.set_logical_size(SCREEN_WIDTH, SCREEN_HEIGHT).expect("Could not set logical size");
        canvas.set_clip_rect(SDLRect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT));

        let event_pump = sdl_context.event_pump()?;
        let keyboard_util = sdl_context.keyboard();

        Ok(RenderContext {
            _image_context,
            canvas,
            event_pump,
            keyboard_util,
        })
    }
}
