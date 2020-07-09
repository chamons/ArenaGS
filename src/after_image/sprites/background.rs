use std::path::Path;

use super::Sprite;
use crate::after_image::{load_image, RenderContext};

use std::cmp;

use crate::atlas::{get_exe_folder, BoxResult};

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

pub struct Background {
    image: Texture,
}

impl Background {
    pub fn init(render_context: &RenderContext, name: &str) -> BoxResult<Background> {
        let map_path = Path::new(&get_exe_folder()).join("maps").join(name).join("map1.png");
        let image = load_image(map_path.to_str().unwrap(), render_context)?;
        Ok(Background { image })
    }
}

impl Sprite for Background {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, _: SDLPoint, _: u32, _: u64) -> BoxResult<()> {
        let (screen_x, screen_y) = canvas.viewport().size();
        let map_box = cmp::min(screen_x, screen_y);
        let image_rect = SDLRect::new(50, 50, 540, 540);
        let screen_rect = SDLRect::new(0, 0, map_box, map_box);

        canvas.copy(&self.image, image_rect, screen_rect)?;

        Ok(())
    }
}
