use std::mem;

use sdl2::render::Texture;

use super::{FontColor, FontSize, RenderCanvas, TextRenderer};
use crate::atlas::prelude::*;

const CACHE_SIZE: usize = 32;

pub struct FontCache {
    cache: [Option<Texture>; CACHE_SIZE],
    current: usize,
}

impl FontCache {
    pub fn init() -> FontCache {
        FontCache {
            cache: Default::default(),
            current: 0,
        }
    }

    pub fn get(&mut self, text_renderer: &TextRenderer, canvas: &RenderCanvas, size: FontSize, color: FontColor, text: &str) -> BoxResult<&Texture> {
        let texture = Some(text_renderer.render_texture(canvas, text, size, color)?);
        let old = mem::replace(&mut self.cache[self.current], texture);
        if let Some(old) = old {
            unsafe {
                // Due to unsafe_textures feature texture is not
                // dropped by default. Because most things are created only
                // once (and not text that continually needs to be blit)
                // this makes using SDL sane in rust, but dynamically
                old.destroy();
            }
        }

        let last_index = self.current;
        self.current += 1;
        if self.current >= CACHE_SIZE {
            self.current = 0;
        }

        Ok(&self.cache[last_index].as_ref().unwrap())
    }
}
