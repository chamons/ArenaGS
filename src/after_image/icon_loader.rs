use std::collections::HashMap;
use std::fs;
use std::path::Path;

use sdl2::render::Texture;

use crate::after_image::prelude::*;

use crate::atlas::get_exe_folder;
use crate::atlas::prelude::*;

// IconLoader lazily loads on first access. This means consumers must
// all get all relevant images outside of a render loop (else we die)
pub struct IconLoader {
    images: HashMap<String, String>,
}

impl IconLoader {
    pub fn init_icons() -> IconLoader {
        let mut images = HashMap::new();
        let folder = Path::new(&get_exe_folder()).join("icons").join("game_icons").stringify_owned();
        find_images(&mut images, &folder).expect("IconLoader unable to load icons");

        IconLoader { images }
    }

    pub fn init_symbols() -> IconLoader {
        let mut images = HashMap::new();
        let folder = Path::new(&get_exe_folder()).join("icons").join("lorc").stringify_owned();
        find_images(&mut images, &folder).expect("IconLoader unable to load symbols");

        IconLoader { images }
    }

    pub fn init_ui() -> IconLoader {
        let mut images = HashMap::new();
        let folder = Path::new(&get_exe_folder()).join("ui").stringify_owned();
        find_images(&mut images, &folder).expect("IconLoader unable to load ui images");

        IconLoader { images }
    }

    pub fn init_overlay_icons() -> IconLoader {
        let mut images = HashMap::new();
        let folder = Path::new(&get_exe_folder()).join("images").join("frames").stringify_owned();
        find_images(&mut images, &folder).expect("IconLoader unable to load overlay icons");

        IconLoader { images }
    }

    pub fn get(&self, render_context: &RenderContext, name: &str) -> BoxResult<Texture> {
        let name = name.to_ascii_lowercase();
        if let Some(path) = self.images.get(&name) {
            load_image(path, render_context)
        } else {
            Err(Box::from(format!("Unable to load image {}", name)))
        }
    }

    pub fn all_icons() -> Vec<String> {
        let folder = Path::new(&get_exe_folder()).join("icons").join("game_icons").stringify_owned();
        let mut filenames = vec![];
        find_filenames(&mut filenames, &folder).expect("Unable to load icon filenames");
        filenames
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

fn find_filenames(filenames: &mut Vec<String>, location: &str) -> BoxResult<()> {
    let entries = fs::read_dir(location)?;
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            find_filenames(filenames, &Path::new(location).join(path).stringify())?;
        } else {
            filenames.push(path.file_name().unwrap().stringify().to_ascii_lowercase().to_string());
        }
    }

    Ok(())
}
