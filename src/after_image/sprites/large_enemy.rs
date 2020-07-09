use std::path::Path;

use super::{Sprite, SpriteFolderDescription};
use crate::after_image::{load_image, RenderContext};
use crate::atlas::BoxResult;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

pub struct LargeEnemy {
    texture: Texture,
}

impl LargeEnemy {
    pub fn init(render_context: &RenderContext, description: &SpriteFolderDescription) -> BoxResult<LargeEnemy> {
        let folder = Path::new(&description.base_folder)
            .join("monsters")
            .join(format!("{}{}", &description.character, ".png"))
            .to_str()
            .unwrap()
            .to_string();

        Ok(LargeEnemy {
            texture: load_image(&folder, render_context)?,
        })
    }
}

impl Sprite for LargeEnemy {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, screen_position: SDLPoint, _: u32, frame: u64) -> BoxResult<()> {
        let offset = super::sprite::get_animation_frame(frame);

        let mut screen_rect = SDLRect::from_center(screen_position, 122, 96);
        // Tweak location a tad from default
        screen_rect.set_x(screen_rect.x() + 1);
        screen_rect.set_y(screen_rect.y() - 5);

        canvas.copy(&self.texture, SDLRect::new(122 * offset as i32, 0, 122, 96), screen_rect)?;

        Ok(())
    }
}
