use std::cmp;
use std::path::Path;

use crate::after_image::{load_image, RenderCanvas, RenderContext};
use crate::atlas::prelude::*;

use sdl2::rect::Point as SDLPoint;
use sdl2::render::Texture;

pub trait Sprite {
    fn draw(&self, canvas: &mut RenderCanvas, screen_position: SDLPoint, state: u32, frame: u64) -> BoxResult<()>;

    fn get_animation_frame(&self, number_of_frames: usize, animation_length: usize, current_frame: u64) -> usize {
        let period = animation_length / number_of_frames;
        let current_frame = current_frame as usize % animation_length;
        let current_frame = (current_frame / period) as usize;
        cmp::min(current_frame, number_of_frames - 1)
    }
}

pub struct SpriteFolderDescription {
    pub base_folder: String,
    pub set: String,
    pub character: String,
}

impl SpriteFolderDescription {
    pub fn init(base_folder: &Path, set: &str, character: &str) -> SpriteFolderDescription {
        SpriteFolderDescription {
            base_folder: base_folder.stringify_owned(),
            set: set.to_string(),
            character: character.to_string(),
        }
    }

    pub fn init_without_set(base_folder: &Path, character: &str) -> SpriteFolderDescription {
        SpriteFolderDescription::init(base_folder, "", character)
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
        .stringify_owned()
}
