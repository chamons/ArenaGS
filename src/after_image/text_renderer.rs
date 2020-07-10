use std::path::PathBuf;

use sdl2::pixels::Color;
use sdl2::rect::Rect as SDLRect;

use super::{FontContext, RenderCanvas};
use crate::atlas::{get_exe_folder, BoxResult};

pub struct TextRenderer<'a> {
    font: sdl2::ttf::Font<'a, 'a>,
}

impl<'a> TextRenderer<'a> {
    pub fn init(font_context: &'a FontContext) -> BoxResult<TextRenderer> {
        let font_path = PathBuf::from(get_exe_folder()).join("fonts").join("LibreFranklin-Regular.ttf");
        let mut font = font_context.ttf_context.load_font(font_path, 20)?;
        font.set_style(sdl2::ttf::FontStyle::NORMAL);

        Ok(TextRenderer { font })
    }

    pub fn render_text(&self, text: &str, x: i32, y: i32, canvas: &mut RenderCanvas) -> BoxResult<()> {
        let surface = self.font.render(text).blended(Color::RGBA(0, 0, 0, 255)).map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();

        let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        let (text_width, text_height) = surface.size();
        canvas.copy(
            &texture,
            SDLRect::new(0, 0, text_width, text_height),
            SDLRect::new(x, y, text_width, text_height),
        )?;

        Ok(())
    }
}
