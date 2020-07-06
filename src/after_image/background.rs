use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use super::{load_image, RenderContext};

use crate::atlas::{get_exe_folder, BoxResult};

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

pub struct Background {
    image: Texture,
}

impl Background {
    pub fn init(name: &str, render_context: &RenderContext) -> BoxResult<Background> {
        let map_path = Path::new(&get_exe_folder()).join("maps").join(name).join("map1.png");
        let image = load_image(map_path.to_str().unwrap(), render_context)?;
        Ok(Background { image })
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, screen_position: SDLPoint) -> BoxResult<()> {
        let image_rect = SDLRect::new(screen_position.x, screen_position.y, 300, 300);
        let screen_rect = SDLRect::new(0, 0, 600, 600);

        canvas.copy(&self.image, image_rect, screen_rect)?;

        Ok(())
    }
}
