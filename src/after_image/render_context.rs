use std::{cell::RefCell, rc::Rc};

use sdl2::image::{self, InitFlag};

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
            .window("Arena GS", 1024, 768)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        let canvas = window.into_canvas().target_texture().present_vsync().build().expect("could not make a canvas");

        let event_pump = sdl_context.event_pump()?;

        Ok(RenderContext {
            _image_context,
            canvas,
            event_pump,
        })
    }
}
