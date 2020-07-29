use std::path::Path;

use super::{Sprite, SpriteFolderDescription};
use crate::after_image::{load_image, RenderCanvas, RenderContext};
use crate::atlas::BoxResult;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

pub struct Bolt {
    texture: Texture,
    start: usize,
    length: usize,
}

impl Bolt {
    pub fn init(render_context: &RenderContext, description: &SpriteFolderDescription) -> BoxResult<Bolt> {
        let folder = Path::new(&description.base_folder)
            .join("bolts")
            .join(format!("{}{}", &description.character, ".png"))
            .to_str()
            .unwrap()
            .to_string();

        Ok(Bolt {
            texture: load_image(&folder, render_context)?,
            start: 0,
            length: 4,
        })
    }
}

impl Sprite for Bolt {
    fn draw(&self, canvas: &mut RenderCanvas, screen_position: SDLPoint, _: u32, frame: u64) -> BoxResult<()> {
        let offset = self.get_animation_frame(self.length, frame);

        let screen_rect = SDLRect::from_center(screen_position, (64.0) as u32, (64.0) as u32);
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
