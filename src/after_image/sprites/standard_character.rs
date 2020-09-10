use std::path::Path;

use super::{Sprite, SpriteFolderDescription};
use crate::after_image::{load_image, RenderCanvas, RenderContext};
use crate::atlas::{BoxResult, EasyPath};

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

pub struct StandardCharacter {
    texture: Texture,
    start: usize,
    render_offset: Option<SDLPoint>,
    scale: f32,
}

impl StandardCharacter {
    pub fn init(render_context: &RenderContext, description: &SpriteFolderDescription, start: usize) -> BoxResult<StandardCharacter> {
        // This is not generic enough
        let folder = Path::new(&description.base_folder)
            .join("monsters")
            .join(format!("{}{}", &description.character, ".png"))
            .stringify_owned();

        Ok(StandardCharacter {
            texture: load_image(&folder, render_context)?,
            start,
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

impl Sprite for StandardCharacter {
    fn draw(&self, canvas: &mut RenderCanvas, screen_position: SDLPoint, _: u32, frame: u64) -> BoxResult<()> {
        let offset = self.get_animation_frame(3, 55, frame);

        let mut screen_rect = SDLRect::from_center(screen_position, (self.scale * 42.0) as u32, (self.scale * 36.0) as u32);
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
    let row = i % 12;
    let col = i / 12;
    SDLRect::new(42 * row as i32, 36 * col as i32, 42, 36)
}
