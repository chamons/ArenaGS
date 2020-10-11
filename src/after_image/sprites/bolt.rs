use std::path::Path;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

use super::{Sprite, SpriteFolderDescription};
use crate::after_image::{load_image, RenderCanvas, RenderContext};
use crate::atlas::prelude::*;

pub struct Bolt {
    texture: Texture,
    start: usize,
    length: usize,
    render_offset: Option<SDLPoint>,
    scale: f32,
}

impl Bolt {
    pub fn init(render_context: &RenderContext, description: &SpriteFolderDescription, start: usize, length: usize) -> BoxResult<Bolt> {
        let folder = Path::new(&description.base_folder)
            .join("bolts")
            .join(format!("{}{}", &description.character, ".png"))
            .stringify_owned();

        Ok(Bolt {
            texture: load_image(&folder, render_context)?,
            start,
            length,
            render_offset: None,
            scale: 1.0,
        })
    }

    pub fn with_render_offset(mut self, render_offset: SDLPoint) -> Self {
        self.render_offset = Some(render_offset);
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

impl Sprite for Bolt {
    fn draw(&self, canvas: &mut RenderCanvas, screen_position: SDLPoint, _: u32, frame: u64) -> BoxResult<()> {
        let offset = self.get_animation_frame(self.length, 30, frame);

        let mut screen_rect = SDLRect::from_center(screen_position, (self.scale * 64.0) as u32, (self.scale * 64.0) as u32);
        if let Some(render_offset) = self.render_offset {
            screen_rect.set_x(screen_rect.x() + render_offset.x());
            screen_rect.set_y(screen_rect.y() + render_offset.y());
        }

        let sprite_rect = get_sprite_sheet_rect_for_index(self.start + offset);

        canvas.copy(&self.texture, sprite_rect, screen_rect)?;

        Ok(())
    }
}

fn get_sprite_sheet_rect_for_index(i: usize) -> SDLRect {
    // Bolt sheets are 5x6 (64x64)
    let row = i % 5;
    let col = i / 5;
    SDLRect::new(64 * row as i32, 64 * col as i32, 64, 64)
}
