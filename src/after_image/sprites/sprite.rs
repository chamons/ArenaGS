use std::path::Path;

use crate::after_image::{load_image, RenderContext};
use crate::atlas::BoxResult;

use sdl2::rect::Point as SDLPoint;
use sdl2::render::Texture;

pub enum SpriteState {
    DetailedCharacter(super::CharacterAnimationState),
    None(),
}

pub trait Sprite {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, screen_position: SDLPoint, state: &SpriteState, frame: u64) -> BoxResult<()>;
}

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
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_animation_frame(frame: u64) -> usize {
    const ANIMATION_LENGTH: usize = 55;
    let frame: usize = frame as usize % ANIMATION_LENGTH;
    if frame > ((2 * ANIMATION_LENGTH) / 3) {
        2
    } else if frame > (ANIMATION_LENGTH / 3) {
        1
    } else {
        0
    }
}
