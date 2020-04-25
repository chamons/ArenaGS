use std::path::Path;

use super::{load_image, BoxResult, RenderContext};

use sdl2::render::Texture;

pub struct SpriteDeepFolderDescription {
    base_folder: String,
    set: String,
    character: String,
}

impl SpriteDeepFolderDescription {
    pub fn init(base_folder: &str, set: &str, character: &str) -> SpriteDeepFolderDescription {
        SpriteDeepFolderDescription {
            base_folder: base_folder.to_string(),
            set: set.to_string(),
            character: character.to_string(),
        }
    }
}

pub struct DetailedCharacterSprite {
    attack_one: [Texture; 3],
    attack_two: [Texture; 3],
    bow: [Texture; 3],
    cheer: [Texture; 3],
    crouch: [Texture; 3],
    down: Texture,
    hit: [Texture; 3],
    idle_one: [Texture; 3],
    item: [Texture; 3],
    magic: [Texture; 3],
    status: [Texture; 3],
    walk: [Texture; 3],
}

impl DetailedCharacterSprite {
    pub fn init(render_context: &RenderContext, description: &SpriteDeepFolderDescription) -> BoxResult<DetailedCharacterSprite> {
        let folder = Path::new(&description.base_folder)
            .join(format!("set{}", &description.set))
            .join(&description.character)
            .to_str()
            .unwrap()
            .to_string();

        Ok(DetailedCharacterSprite {
            attack_one: load_set(&folder, description, "atk1", render_context)?,
            attack_two: load_set(&folder, description, "atk2", render_context)?,
            bow: load_set(&folder, description, "bow", render_context)?,
            cheer: load_set(&folder, description, "cheer", render_context)?,
            crouch: load_set(&folder, description, "crouch", render_context)?,
            hit: load_set(&folder, description, "hit", render_context)?,
            idle_one: load_set(&folder, description, "idle1", render_context)?,
            item: load_set(&folder, description, "item", render_context)?,
            magic: load_set(&folder, description, "magic", render_context)?,
            status: load_set(&folder, description, "status", render_context)?,
            walk: load_set(&folder, description, "walk", render_context)?,
            down: load_image(&get_single_name(&folder, description), render_context)?,
        })
    }

    pub fn get_texture(&self) -> &Texture {
        &self.idle_one[0]
    }
}

fn load_set(folder: &str, description: &SpriteDeepFolderDescription, action: &str, render_context: &RenderContext) -> BoxResult<[Texture; 3]> {
    let first = load_image(&get_set_name(folder, description, action, "1"), render_context)?;
    let second = load_image(&get_set_name(folder, description, action, "2"), render_context)?;
    let third = load_image(&get_set_name(folder, description, action, "3"), render_context)?;
    Ok([first, second, third])
}

fn get_single_name(folder: &str, description: &SpriteDeepFolderDescription) -> String {
    Path::new(&folder)
        .join(format!("{}_{}_down.png", description.set, description.character))
        .to_str()
        .unwrap()
        .to_string()
}

fn get_set_name(folder: &str, description: &SpriteDeepFolderDescription, action: &str, index: &str) -> String {
    Path::new(&folder)
        .join(format!("{}_{}_{} ({}).png", description.set, description.character, action, index))
        .to_str()
        .unwrap()
        .to_string()
}
