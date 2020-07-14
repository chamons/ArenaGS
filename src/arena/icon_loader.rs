use std::collections::HashMap;
use std::fs;
use std::path::Path;

use sdl2::render::Texture;

use crate::after_image::{load_image, RenderContext};

use crate::atlas::{get_exe_folder, BoxResult};

pub struct IconLoader {
    images: HashMap<String, Texture>,
}

impl IconLoader {
    pub fn init(render_context: &RenderContext) -> BoxResult<IconLoader> {
        let mut images = HashMap::new();
        let folder = Path::new(&get_exe_folder()).join("icons").join("game_icons").to_str().unwrap().to_string();
        find_images(render_context, &mut images, &folder)?;

        Ok(IconLoader { images })
    }
}

fn find_images(render_context: &RenderContext, images: &mut HashMap<String, Texture>, location: &str) -> BoxResult<()> {
    let entries = fs::read_dir(location)?;
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            find_images(render_context, images, &Path::new(location).join(path).to_str().unwrap())?;
        } else {
            let path = path.to_str().unwrap();
            let image = load_image(path, render_context)?;
            images.insert(path.to_string(), image);
        }
    }
    Ok(())
}
