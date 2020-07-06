use std::path::Path;

use super::{load_image, RenderContext};
use crate::atlas::BoxResult;

use sdl2::render::Texture;

pub struct SpriteFolderDescription {
    pub base_folder: String,
    pub set: String,
    pub character: String,
}

impl SpriteFolderDescription {
    pub fn init(base_folder: &Path, set: &str, character: &str) -> SpriteFolderDescription {
        SpriteFolderDescription {
            base_folder: base_folder.to_str().unwrap().to_string(),
            set: set.to_string(),
            character: character.to_string(),
        }
    }
}

pub fn load_set(folder: &str, description: &SpriteFolderDescription, action: &str, render_context: &RenderContext) -> BoxResult<[Texture; 3]> {
    let first = load_image(&get_set_name(folder, description, action, "1"), render_context)?;
    let second = load_image(&get_set_name(folder, description, action, "2"), render_context)?;
    let third = load_image(&get_set_name(folder, description, action, "3"), render_context)?;
    Ok([first, second, third])
}

fn get_set_name(folder: &str, description: &SpriteFolderDescription, action: &str, index: &str) -> String {
    Path::new(&folder)
        .join(format!("{}_{}_{} ({}).png", description.set, description.character, action, index))
        .to_str()
        .unwrap()
        .to_string()
}
