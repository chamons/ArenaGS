use super::RenderContext;
use crate::atlas::{get_exe_folder, BoxResult};

use std::path::Path;

use sdl2::image::LoadTexture;
use sdl2::render::Texture;

pub fn load_image(path: &str, render_context: &RenderContext) -> BoxResult<Texture> {
    let dest_path = Path::new(&get_exe_folder()).join(path);

    let texture_creator = render_context.canvas.texture_creator();
    Ok(texture_creator.load_texture(dest_path)?)
}
