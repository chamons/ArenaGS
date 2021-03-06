use std::cell::RefCell;
use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::{Texture, TextureQuery};

use super::{FontCache, FontContext, LayoutRequest, LayoutResult, RenderCanvas};
use crate::atlas::get_exe_folder;
use crate::atlas::prelude::*;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum FontSize {
    Micro,
    VeryTiny,
    Tiny,
    Small,
    SmallUnderline,
    Large,
    Bold,
}

impl FontSize {
    pub fn smaller(&self) -> FontSize {
        match self {
            FontSize::Micro | FontSize::SmallUnderline => panic!("FontSize {:?} does not have smaller version", self),
            FontSize::Bold => FontSize::Small,
            FontSize::Large => FontSize::Small,
            FontSize::Small => FontSize::Tiny,
            FontSize::Tiny => FontSize::VeryTiny,
            FontSize::VeryTiny => FontSize::Micro,
        }
    }
}

#[derive(Copy, Clone)]
pub enum FontColor {
    Black,
    White,
    Red,
    LightBrown,
    Brown,
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
    very_tiny_font: Font,
    tiny_font: Font,
    small_font: Font,
    small_underline_font: Font,
    bold_font: Font,
    large_font: Font,
}

impl TextRenderer {
    pub fn init(font_context: &'static FontContext) -> BoxResult<TextRenderer> {
        let font_path = Path::new(&get_exe_folder()).join("fonts").join("LibreFranklin-Regular.ttf");

        let mut micro_font = font_context.ttf_context.load_font(font_path.clone(), 9)?;
        micro_font.set_style(sdl2::ttf::FontStyle::BOLD);
        let mut very_tiny_font = font_context.ttf_context.load_font(font_path.clone(), 11)?;
        very_tiny_font.set_style(sdl2::ttf::FontStyle::NORMAL);
        let mut tiny_font = font_context.ttf_context.load_font(font_path.clone(), 12)?;
        tiny_font.set_style(sdl2::ttf::FontStyle::NORMAL);
        let mut small_font = font_context.ttf_context.load_font(font_path.clone(), 14)?;
        small_font.set_style(sdl2::ttf::FontStyle::NORMAL);
        let mut small_underline_font = font_context.ttf_context.load_font(font_path.clone(), 14)?;
        small_underline_font.set_style(sdl2::ttf::FontStyle::UNDERLINE);
        let mut bold_font = font_context.ttf_context.load_font(font_path.clone(), 16)?;
        bold_font.set_style(sdl2::ttf::FontStyle::BOLD);
        let mut large_font = font_context.ttf_context.load_font(font_path, 20)?;
        large_font.set_style(sdl2::ttf::FontStyle::BOLD);

        Ok(TextRenderer {
            cache: RefCell::new(FontCache::init()),
            micro_font,
            very_tiny_font,
            tiny_font,
            small_font,
            small_underline_font,
            bold_font,
            large_font,
        })
    }

    fn get_font(&self, size: FontSize) -> &Font {
        match size {
            FontSize::Micro => &self.micro_font,
            FontSize::VeryTiny => &self.very_tiny_font,
            FontSize::Tiny => &self.tiny_font,
            FontSize::Small => &self.small_font,
            FontSize::SmallUnderline => &self.small_underline_font,
            FontSize::Bold => &self.bold_font,
            FontSize::Large => &self.large_font,
        }
    }

    pub fn layout_text(&self, text: &str, size: FontSize, request: LayoutRequest) -> BoxResult<LayoutResult> {
        super::text_layout::layout_text(text, self.get_font(size), request)
    }

    pub fn render_texture(&self, canvas: &RenderCanvas, text: &str, size: FontSize, color: FontColor) -> BoxResult<Texture> {
        let color = match color {
            FontColor::Black => Color::RGB(0, 0, 0),
            FontColor::White => Color::RGB(255, 255, 255),
            FontColor::Red => Color::RGB(255, 0, 0),
            FontColor::Brown => Color::RGB(73, 54, 41),
            FontColor::LightBrown => Color::RGB(115, 96, 76),
        };

        let surface = self.get_font(size).render(text).blended(color).map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        Ok(texture)
    }

    pub fn render_text(&self, text: &str, x: i32, y: i32, canvas: &mut RenderCanvas, size: FontSize, color: FontColor) -> BoxResult<(u32, u32)> {
        let mut cache = self.cache.borrow_mut();
        let texture = cache.get(&self, canvas, size, color, text)?;
        let TextureQuery { width, height, .. } = texture.query();
        canvas.copy(&texture, SDLRect::new(0, 0, width, height), SDLRect::new(x, y, width, height))?;

        #[cfg(feature = "debug_text_alignmnet")]
        {
            canvas.set_draw_color(Color::from((128, 0, 0, 128)));
            canvas.fill_rect(SDLRect::new(x, y, width, height))?;
        }

        Ok((width, height))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_text_centered(
        &self,
        text: &str,
        x: i32,
        y: i32,
        text_render_width: u32,
        canvas: &mut RenderCanvas,
        size: FontSize,
        color: FontColor,
    ) -> BoxResult<(u32, u32)> {
        let text_x = {
            let width = {
                let mut cache = self.cache.borrow_mut();
                let texture = cache.get(&self, canvas, size, color, text)?;
                let TextureQuery { width, .. } = texture.query();
                width
            };
            if width > text_render_width {
                return self.render_text_centered(text, x, y, text_render_width, canvas, size.smaller(), color);
            }
            (text_render_width - width) / 2
        };

        self.render_text(text, x + text_x as i32, y, canvas, size, color)
    }
}
