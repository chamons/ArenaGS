use super::{BoxResult, RenderContext};

use sdl2::image::LoadTexture;
use sdl2::render::Texture;

fn get_exe_folder() -> BoxResult<std::path::PathBuf> {
    let exe_path = std::env::current_exe()?;
    let exe_folder = exe_path.parent().ok_or_else(|| ("No Parent Exe Folder"))?;
    Ok(exe_folder.to_path_buf())
}

pub fn load_image(path: &str, render_context: &RenderContext) -> BoxResult<Texture> {
    let dest_path = get_exe_folder()?.join(path);

    let texture_creator = render_context.canvas.texture_creator();
    Ok(texture_creator.load_texture(dest_path)?)
}
