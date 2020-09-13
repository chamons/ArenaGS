use std::path::Path;

use super::{Sprite, SpriteFolderDescription};
use crate::after_image::{load_image, RenderCanvas, RenderContext};
use crate::atlas::{BoxResult, EasyPath};

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

pub enum StandardCharacterSize {
    Micro,
    Normal,
}

pub struct StandardCharacter {
    texture: Texture,
    start: usize,
    size: StandardCharacterSize,
}

impl StandardCharacter {
    pub fn init(
        render_context: &RenderContext,
        description: &SpriteFolderDescription,
        start: usize,
        size: StandardCharacterSize,
    ) -> BoxResult<StandardCharacter> {
        // This is not generic enough
        let folder = Path::new(&description.base_folder)
            .join("monsters")
            .join(format!("{}{}", &description.character, ".png"))
            .stringify_owned();

        Ok(StandardCharacter {
            texture: load_image(&folder, render_context)?,
            start,
            size,
        })
    }

    fn get_scale(&self) -> f32 {
        match self.size {
            StandardCharacterSize::Micro => 2.5,
            StandardCharacterSize::Normal => 1.0,
        }
    }

    fn get_offset(&self) -> Option<SDLPoint> {
        match self.size {
            StandardCharacterSize::Micro => Some(SDLPoint::new(0, -8)),
            StandardCharacterSize::Normal => None,
        }
    }
}

impl Sprite for StandardCharacter {
    fn draw(&self, canvas: &mut RenderCanvas, screen_position: SDLPoint, _: u32, frame: u64) -> BoxResult<()> {
        let offset = self.get_animation_frame(3, 55, frame);

        let scale = self.get_scale();
        let mut screen_rect = SDLRect::from_center(screen_position, (scale * 42.0) as u32, (scale * 36.0) as u32);
        if let Some(render_offset) = self.get_offset() {
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
