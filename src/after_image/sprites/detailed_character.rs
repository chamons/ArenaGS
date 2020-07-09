use std::path::Path;

use num_derive::FromPrimitive;
use num_enum::IntoPrimitive;
use num_traits::FromPrimitive;

use super::{sprite::load_set, SpriteFolderDescription};
use crate::after_image::{load_image, RenderContext, Sprite};

use crate::atlas::BoxResult;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

#[allow(dead_code)]
#[derive(IntoPrimitive, FromPrimitive)]
#[repr(u32)]
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

pub struct DetailedCharacter {
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

impl DetailedCharacter {
    pub fn init(render_context: &RenderContext, description: &SpriteFolderDescription) -> BoxResult<DetailedCharacter> {
        let folder = Path::new(&description.base_folder)
            .join("battle")
            .join(format!("set{}", &description.set))
            .join(&description.character)
            .to_str()
            .unwrap()
            .to_string();

        Ok(DetailedCharacter {
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

    fn get_texture(&self, state: &CharacterAnimationState, frame: u64) -> &Texture {
        let offset = super::sprite::get_animation_frame(frame);

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
}

impl Sprite for DetailedCharacter {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, screen_position: SDLPoint, state: u32, frame: u64) -> BoxResult<()> {
        if let Some(state) = CharacterAnimationState::from_u32(state) {
            let screen_rect = SDLRect::from_center(screen_position, 96, 96);
            canvas.copy(self.get_texture(&state, frame), SDLRect::new(0, 0, 96, 96), screen_rect)?;
        }
        Ok(())
    }
}

pub fn get_single_name(folder: &str, description: &SpriteFolderDescription) -> String {
    Path::new(&folder)
        .join(format!("{}_{}_down.png", description.set, description.character))
        .to_str()
        .unwrap()
        .to_string()
}
