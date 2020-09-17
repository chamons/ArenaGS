use std::collections::HashMap;
use std::fs;
use std::path::Path;

use sdl2::render::Texture;

use crate::after_image::{load_image, RenderContext};

use crate::atlas::{get_exe_folder, BoxResult, EasyPath};

// UILoader eagerly loads all images in the ui directory
// as knowing what components of the UI you'll need is tricky
pub struct UILoader {
    images: HashMap<String, Texture>,
}

impl UILoader {
    pub fn init(render_context: &RenderContext) -> BoxResult<UILoader> {
        let mut images = HashMap::new();
        let folder = Path::new(&get_exe_folder()).join("ui").stringify_owned();
        load_images(render_context, &mut images, &folder)?;

        Ok(UILoader { images })
    }

    pub fn get(&self, name: &str) -> &Texture {
        let name = name.to_ascii_lowercase();
        self.images.get(&name).unwrap()
    }
}

fn load_images(render_context: &RenderContext, images: &mut HashMap<String, Texture>, location: &str) -> BoxResult<()> {
    let entries = fs::read_dir(location)?;
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            load_images(render_context, images, &Path::new(location).join(path).stringify())?;
        } else {
            let name = path.file_name().unwrap().stringify().to_ascii_lowercase();
            if images.contains_key(&name) {
                println!("UILoader Warning: {} already exists!", name)
            }
            images.insert(name.to_string(), load_image(path.stringify(), render_context)?);
        }
    }
    Ok(())
}
