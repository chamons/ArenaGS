use std::collections::HashMap;
use std::fs;
use std::path::Path;

use sdl2::render::Texture;

use crate::after_image::{load_image, RenderContext};

use crate::atlas::{get_exe_folder, BoxResult, EasyPath};

pub struct IconLoader {
    images: HashMap<String, String>,
}

impl IconLoader {
    pub fn init(subfolder: &str) -> BoxResult<IconLoader> {
        let mut images = HashMap::new();
        let folder = Path::new(&get_exe_folder()).join("icons").join("game_icons").join(subfolder).stringify_owned();
        find_images(&mut images, &folder)?;

        Ok(IconLoader { images })
    }

    pub fn get(&self, render_context: &RenderContext, name: &str) -> BoxResult<Texture> {
        let name = name.to_ascii_lowercase();
        if let Some(path) = self.images.get(&name) {
            load_image(path, render_context)
        } else {
            Err(Box::from(format!("Unable to load image {}", name)))
        }
    }
}

fn find_images(images: &mut HashMap<String, String>, location: &str) -> BoxResult<()> {
    let entries = fs::read_dir(location)?;
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            find_images(images, &Path::new(location).join(path).stringify())?;
        } else {
            let name = path.file_name().unwrap().stringify().to_ascii_lowercase();
            if images.contains_key(&name) {
                println!("IconLoader Warning: {} already exists!", name)
            }
            images.insert(name.to_string(), path.stringify_owned());
        }
    }
    Ok(())
}
