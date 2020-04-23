use super::RenderContext;
use sdl2::render::Texture;

use sdl2::image::LoadTexture;

pub fn load_image<'r>(path: &str, render_context: &RenderContext) -> Result<Texture, String> {
    // HACK
    let data_path = format!("{}\\..\\ArenaGS-Data", env!("CARGO_MANIFEST_DIR"));

    let texture_creator = render_context.canvas.texture_creator();
    Ok(texture_creator.load_texture(format!("{}{}", data_path, path))?)
}
