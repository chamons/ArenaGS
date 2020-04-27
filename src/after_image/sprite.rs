use std::path::Path;

use super::{load_image, RenderContext};
use crate::atlas::BoxResult;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

pub struct SpriteDeepFolderDescription {
    base_folder: String,
    set: String,
    character: String,
}

impl SpriteDeepFolderDescription {
    pub fn init(base_folder: &Path, set: &str, character: &str) -> SpriteDeepFolderDescription {
        SpriteDeepFolderDescription {
            base_folder: base_folder.to_str().unwrap().to_string(),
            set: set.to_string(),
            character: character.to_string(),
        }
    }
}

#[allow(dead_code)]
pub enum CharacterAnimationState {
    AttackOne,
    AttackTwo,
    Bow,
    Cheer,
    Crouch,
    Down,
    Hit,
    Idle,
    Item,
    Magic,
    Status,
    Walk,
}

pub struct DetailedCharacterSprite {
    attack_one: [Texture; 3],
    attack_two: [Texture; 3],
    bow: [Texture; 3],
    cheer: [Texture; 3],
    crouch: [Texture; 3],
    down: Texture,
    hit: [Texture; 3],
    idle: [Texture; 3],
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
            idle: load_set(&folder, description, "idle1", render_context)?,
            item: load_set(&folder, description, "item", render_context)?,
            magic: load_set(&folder, description, "magic", render_context)?,
            status: load_set(&folder, description, "status", render_context)?,
            walk: load_set(&folder, description, "walk", render_context)?,
            down: load_image(&get_single_name(&folder, description), render_context)?,
        })
    }

    fn get_texture(&self, state: CharacterAnimationState, frame: u64) -> &Texture {
        const ANIMATION_LENGTH: usize = 55;
        let frame: usize = frame as usize % ANIMATION_LENGTH;
        let offset = if frame > ((2 * ANIMATION_LENGTH) / 3) {
            2
        } else if frame > (ANIMATION_LENGTH / 3) {
            1
        } else {
            0
        };

        match state {
            CharacterAnimationState::AttackOne => &self.attack_one[offset],
            CharacterAnimationState::AttackTwo => &self.attack_two[offset],
            CharacterAnimationState::Bow => &self.bow[offset],
            CharacterAnimationState::Cheer => &self.cheer[offset],
            CharacterAnimationState::Crouch => &self.crouch[offset],
            CharacterAnimationState::Down => &self.down,
            CharacterAnimationState::Hit => &self.hit[offset],
            CharacterAnimationState::Idle => &self.idle[offset],
            CharacterAnimationState::Item => &self.item[offset],
            CharacterAnimationState::Magic => &self.magic[offset],
            CharacterAnimationState::Status => &self.status[offset],
            CharacterAnimationState::Walk => &self.walk[offset],
        }
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        screen_position: SDLPoint,
        state: CharacterAnimationState,
        frame: u64,
    ) -> BoxResult<()> {
        let screen_rect = SDLRect::from_center(screen_position, 96, 96);
        canvas.copy(self.get_texture(CharacterAnimationState::Idle, frame), SDLRect::new(0, 0, 96, 96), screen_rect)?;

        Ok(())
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
