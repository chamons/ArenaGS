use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

use super::{FontContext, RenderCanvas};
use crate::atlas::{get_exe_folder, BoxResult};

pub enum FontSize {
    Small,
    Large,
    Bold,
}

pub enum FontColor {
    Black,
    White,
    Red,
}

pub struct TextRenderer<'a> {
    small_font: sdl2::ttf::Font<'a, 'a>,
    bold_font: sdl2::ttf::Font<'a, 'a>,
    large_font: sdl2::ttf::Font<'a, 'a>,
}

impl<'a> TextRenderer<'a> {
    pub fn init(font_context: &'a FontContext) -> BoxResult<TextRenderer> {
        let font_path = Path::new(&get_exe_folder()).join("fonts").join("LibreFranklin-Regular.ttf");

        let mut small_font = font_context.ttf_context.load_font(font_path.clone(), 14)?;
        small_font.set_style(sdl2::ttf::FontStyle::NORMAL);
        let mut bold_font = font_context.ttf_context.load_font(font_path.clone(), 16)?;
        bold_font.set_style(sdl2::ttf::FontStyle::BOLD);
        let mut large_font = font_context.ttf_context.load_font(font_path, 20)?;
        large_font.set_style(sdl2::ttf::FontStyle::NORMAL);

        Ok(TextRenderer {
            small_font,
            bold_font,
            large_font,
        })
    }

    pub fn render_texture(&self, canvas: &RenderCanvas, text: &str, size: FontSize, color: FontColor) -> BoxResult<((u32, u32), Texture)> {
        let font = match size {
            FontSize::Small => &self.small_font,
            FontSize::Bold => &self.bold_font,
            FontSize::Large => &self.large_font,
        };
        let color = match color {
            FontColor::Black => Color::RGBA(0, 0, 0, 255),
            FontColor::White => Color::RGBA(255, 255, 255, 255),
            FontColor::Red => Color::RGBA(255, 0, 0, 255),
        };

        let surface = font.render(text).blended(color).map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        Ok((surface.size(), texture))
    }

    pub fn render_text(&self, text: &str, x: i32, y: i32, canvas: &mut RenderCanvas, size: FontSize, color: FontColor) -> BoxResult<()> {
        let ((text_width, text_height), texture) = self.render_texture(canvas, text, size, color)?;
        canvas.copy(
            &texture,
            SDLRect::new(0, 0, text_width, text_height),
            SDLRect::new(x, y, text_width, text_height),
        )?;

        Ok(())
    }
}
