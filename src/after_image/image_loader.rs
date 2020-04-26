use super::RenderContext;
use crate::atlas::BoxResult;

use std::path::PathBuf;
use std::path::MAIN_SEPARATOR;

use sdl2::image::LoadTexture;
use sdl2::render::Texture;

fn get_exe_folder() -> String {
    let exe = std::env::current_exe().unwrap();
    let exe_path = exe.to_str().unwrap();
    let mut bits: Vec<&str> = exe_path.split(MAIN_SEPARATOR).collect();
    bits.pop();
    bits.join(&MAIN_SEPARATOR.to_string()).to_string()
}

pub fn load_image(path: &str, render_context: &RenderContext) -> BoxResult<Texture> {
    let dest_path = PathBuf::from(get_exe_folder()).join(path);

    let texture_creator = render_context.canvas.texture_creator();
    Ok(texture_creator.load_texture(dest_path)?)
}
