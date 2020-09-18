use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

use super::{FontContext, RenderCanvas};
use crate::atlas::{get_exe_folder, BoxResult};

#[allow(dead_code)]
pub enum FontSize {
    Micro,
    Small,
    Large,
    Bold,
}

pub enum FontColor {
    Black,
    White,
    Red,
}

// So this is either a beautiful hack, or an abuse
// The SDL font code wants two lifetimes, which requires
// TextRenderer to have a lifeime, which causes a bunch
// of other classes to require lifetime
// By just leaking the fonts, which last the lifetime of
// the game, all of this goes away.
// This is safe, right?!?
pub struct TextRenderer {
    micro_font: sdl2::ttf::Font<'static, 'static>,
    small_font: sdl2::ttf::Font<'static, 'static>,
    bold_font: sdl2::ttf::Font<'static, 'static>,
    large_font: sdl2::ttf::Font<'static, 'static>,
}

impl TextRenderer {
    pub fn init(font_context: &'static FontContext) -> BoxResult<TextRenderer> {
        let font_path = Path::new(&get_exe_folder()).join("fonts").join("LibreFranklin-Regular.ttf");

        let mut micro_font = font_context.ttf_context.load_font(font_path.clone(), 9)?;
        micro_font.set_style(sdl2::ttf::FontStyle::BOLD);
        let mut small_font = font_context.ttf_context.load_font(font_path.clone(), 14)?;
        small_font.set_style(sdl2::ttf::FontStyle::NORMAL);
        let mut bold_font = font_context.ttf_context.load_font(font_path.clone(), 16)?;
        bold_font.set_style(sdl2::ttf::FontStyle::BOLD);
        let mut large_font = font_context.ttf_context.load_font(font_path, 20)?;
        large_font.set_style(sdl2::ttf::FontStyle::NORMAL);

        Ok(TextRenderer {
            micro_font,
            small_font,
            bold_font,
            large_font,
        })
    }

    pub fn render_texture(&self, canvas: &RenderCanvas, text: &str, size: FontSize, color: FontColor) -> BoxResult<((u32, u32), Texture)> {
        let font = match size {
            FontSize::Micro => &self.micro_font,
            FontSize::Small => &self.small_font,
            FontSize::Bold => &self.bold_font,
            FontSize::Large => &self.large_font,
        };
        let color = match color {
            FontColor::Black => Color::RGB(0, 0, 0),
            FontColor::White => Color::RGB(255, 255, 255),
            FontColor::Red => Color::RGB(255, 0, 0),
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

        // Due to unsafe_textures feature texture is not
        // dropped by default. Because most things are created only
        // once (and not text that continually needs to be blit)
        // this makes using SDL sane in rust, but dynamically
        // created content such as texts must handle this
        unsafe {
            texture.destroy();
        }

        Ok(())
    }
}
