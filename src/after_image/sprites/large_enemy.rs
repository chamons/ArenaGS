use std::path::Path;

use super::{Sprite, SpriteFolderDescription};
use crate::after_image::{load_image, RenderCanvas, RenderContext};
use crate::atlas::{BoxResult, EasyPath};

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

pub struct LargeEnemy {
    texture: Texture,
    scale: f32,
}

impl LargeEnemy {
    pub fn init(render_context: &RenderContext, description: &SpriteFolderDescription, scale: f32) -> BoxResult<LargeEnemy> {
        let folder = Path::new(&description.base_folder)
            .join("monsters")
            .join(format!("{}{}", &description.character, ".png"))
            .stringify_owned();

        Ok(LargeEnemy {
            texture: load_image(&folder, render_context)?,
            scale,
        })
    }
}

impl Sprite for LargeEnemy {
    fn draw(&self, canvas: &mut RenderCanvas, screen_position: SDLPoint, _: u32, frame: u64) -> BoxResult<()> {
        let offset = self.get_animation_frame(3, 55, frame);

        let mut screen_rect = SDLRect::from_center(screen_position, (122.0 * self.scale) as u32, (96.0 * self.scale) as u32);
        // Tweak location a tad from default
        screen_rect.set_x(screen_rect.x() + 1);
        screen_rect.set_y(screen_rect.y() - 20);

        canvas.copy(&self.texture, SDLRect::new(122 * offset as i32, 0, 122, 96), screen_rect)?;

        Ok(())
    }
}
