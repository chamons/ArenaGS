use std::cell::RefCell;
use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::{Texture, TextureQuery};

use super::{FontCache, FontContext, RenderCanvas};
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

pub type Font = sdl2::ttf::Font<'static, 'static>;

// So this is either a beautiful hack, or an abuse
// The SDL font code wants two lifetimes, which requires
// TextRenderer to have a lifeime, which causes a bunch
// of other classes to require lifetime
// By just leaking the fonts, which last the lifetime of
// the game, all of this goes away.
// This is safe, right?!?
pub struct TextRenderer {
    cache: RefCell<FontCache>,
    micro_font: Font,
    small_font: Font,
    bold_font: Font,
    large_font: Font,
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
            cache: RefCell::new(FontCache::init()),
            micro_font,
            small_font,
            bold_font,
            large_font,
        })
    }

    pub fn render_texture(&self, canvas: &RenderCanvas, text: &str, size: FontSize, color: FontColor) -> BoxResult<Texture> {
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

        let surface = font.render(text).solid(color).map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        Ok(texture)
    }

    pub fn render_text(&self, text: &str, x: i32, y: i32, canvas: &mut RenderCanvas, size: FontSize, color: FontColor) -> BoxResult<()> {
        let mut cache = self.cache.borrow_mut();
        let texture = cache.get(&self, canvas, size, color, text)?;
        let TextureQuery { width, height, .. } = texture.query();
        canvas.copy(&texture, SDLRect::new(0, 0, width, height), SDLRect::new(x, y, width, height))?;

        Ok(())
    }
}
