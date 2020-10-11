use std::cmp;
use std::path::Path;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;

use sdl2::rect::Point as SDLPoint;

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
    pub name: String,
}

impl SpriteFolderDescription {
    pub fn init(base_folder: &Path, name: &str) -> SpriteFolderDescription {
        SpriteFolderDescription {
            base_folder: base_folder.stringify_owned(),
            name: name.to_string(),
        }
    }
}
