use super::RenderContext;

use sdl2::image::LoadTexture;
use sdl2::render::Texture;

fn get_exe_folder() -> Option<std::path::PathBuf> {
    let exe_path = std::env::current_exe().unwrap();
    Some(exe_path.parent()?.to_path_buf())
}

pub fn load_image<'r>(path: &str, render_context: &RenderContext) -> Result<Texture, String> {
    let dest_path = get_exe_folder().unwrap().join(path);

    let texture_creator = render_context.canvas.texture_creator();
    Ok(texture_creator.load_texture(dest_path)?)
}
