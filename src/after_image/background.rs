use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use super::{load_image, RenderContext};

use crate::atlas::{get_exe_folder, BoxResult};

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

use tiled::parse_file;

pub struct Background {
    map: tiled::Map,
    sprite_sheet: Texture,
}

impl Background {
    pub fn init(name: &str, render_context: &RenderContext) -> BoxResult<Background> {
        let map_path = Path::new(&get_exe_folder()).join("maps").join(name).join("map.tmx");
        let map = parse_file(&map_path)?;

        let sprite_path = Path::new(&get_exe_folder()).join("maps").join(name).join("tileset.png");
        let sprite_sheet = load_image(sprite_path.to_str().unwrap(), render_context)?;
        Ok(Background { map, sprite_sheet })
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, screen_position: SDLPoint) -> BoxResult<()> {
        let single_tile_width = self.map.tile_width;
        let single_tile_height = self.map.tile_height;
        let columns = 36; // HACK

        for w in 0..18 {
            for h in 0..18 {
                let tile = &self.map.layers[0].tiles[w][h];
                let gid = tile.gid - 1;

                let (x, y) = (gid % columns, gid / columns);
                let sprite_sheet_rect = SDLRect::new(
                    (x * single_tile_width) as i32,
                    (y * single_tile_height) as i32,
                    single_tile_width,
                    single_tile_height,
                );
                let screen_rect = SDLRect::new(
                    (w as u32 * single_tile_width * 2) as i32,
                    (h as u32 * single_tile_height * 2) as i32,
                    single_tile_width * 2,
                    single_tile_height * 2,
                );
                canvas.copy(&self.sprite_sheet, sprite_sheet_rect, screen_rect)?;
            }
        }

        canvas.present();
        Ok(())
    }
}
