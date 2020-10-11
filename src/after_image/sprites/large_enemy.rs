use std::path::Path;

use super::{Sprite, SpriteFolderDescription};
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

pub enum LargeCharacterSize {
    Normal,    // (94, 100)
    Bird,      // (122, 96)
    LargeBird, // (122, 96) scale 1.5
}

pub struct LargeEnemy {
    texture: Texture,
    size: LargeCharacterSize,
}

impl LargeEnemy {
    pub fn init(render_context: &RenderContext, description: &SpriteFolderDescription, size: LargeCharacterSize) -> BoxResult<LargeEnemy> {
        let folder = Path::new(&description.base_folder)
            .join("monsters")
            .join(format!("{}.png", &description.name))
            .stringify_owned();

        Ok(LargeEnemy {
            texture: load_image(&folder, render_context)?,
            size,
        })
    }

    fn get_size(&self) -> (i32, i32) {
        match self.size {
            LargeCharacterSize::Normal => (94, 100),
            LargeCharacterSize::Bird => (122, 96),
            LargeCharacterSize::LargeBird => (122, 96),
        }
    }

    fn get_scale(&self) -> f32 {
        match self.size {
            LargeCharacterSize::LargeBird => 1.5,
            _ => 1.0,
        }
    }

    fn get_offset(&self) -> Option<SDLPoint> {
        match self.size {
            LargeCharacterSize::Normal => None,
            LargeCharacterSize::Bird => Some(SDLPoint::new(1, -20)),
            LargeCharacterSize::LargeBird => Some(SDLPoint::new(1, -20)),
        }
    }
}

impl Sprite for LargeEnemy {
    fn draw(&self, canvas: &mut RenderCanvas, screen_position: SDLPoint, _: u32, frame: u64) -> BoxResult<()> {
        let offset = self.get_animation_frame(3, 55, frame);

        let scale = self.get_scale();
        let (width, height) = self.get_size();

        let mut screen_rect = SDLRect::from_center(screen_position, (width as f32 * scale) as u32, (height as f32 * scale) as u32);
        if let Some(render_offset) = self.get_offset() {
            screen_rect.set_x(screen_rect.x() + render_offset.x());
            screen_rect.set_y(screen_rect.y() + render_offset.y());
        }

        canvas.copy(&self.texture, SDLRect::new(width * offset as i32, 0, width as u32, height as u32), screen_rect)?;

        Ok(())
    }
}
