use std::path::Path;

use num_derive::FromPrimitive;
use num_enum::IntoPrimitive;
use num_traits::FromPrimitive;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;

use super::SpriteFolderDescription;
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum CharacterAnimationState {
    AttackOne,
    AttackTwo,
    Bow,
    Cheer,
    Crouch,
    Hit,
    Idle,
    Item,
    Magic,
    Status,
    Walk,
}

pub struct DetailedCharacter {
    sprites: Texture,
}

impl DetailedCharacter {
    pub fn init(render_context: &RenderContext, description: &SpriteFolderDescription) -> BoxResult<DetailedCharacter> {
        let sheet_path = Path::new(&description.base_folder)
            .join("battle")
            .join(format!("{}.png", &description.name))
            .stringify_owned();

        Ok(DetailedCharacter {
            sprites: load_image(&sheet_path, render_context)?,
        })
    }

    fn get_sprite_sheet_rect_for_index(&self, i: usize) -> SDLRect {
        let (width, height) = (144, 144);
        let row = i % 9;
        let col = i / 9;
        SDLRect::new(width * row as i32, height * col as i32, width as u32, height as u32)
    }

    fn get_texture(&self, state: &CharacterAnimationState, frame: u64) -> (&Texture, SDLRect) {
        let animation_length = match state {
            CharacterAnimationState::Idle => 55,
            _ => 15,
        };
        let offset = self.get_animation_frame(3, animation_length, frame);

        // The detailed character sheets are somewhat strangely laid out
        // 1, 2, 0
        let offset = match offset {
            0 => 2,
            1 => 0,
            2 => 1,
            _ => panic!("Unexpected animation offset"),
        };

        let sprite_index = match state {
            CharacterAnimationState::Idle => 0,
            CharacterAnimationState::AttackOne => 3,
            CharacterAnimationState::Walk => 6,
            CharacterAnimationState::AttackTwo => 12,
            CharacterAnimationState::Cheer => 15,
            CharacterAnimationState::Magic => 18,
            CharacterAnimationState::Bow => 21,
            CharacterAnimationState::Crouch => 24,
            CharacterAnimationState::Hit => 36,
            CharacterAnimationState::Status => 42,
            CharacterAnimationState::Item => 48,
        };

        (&self.sprites, self.get_sprite_sheet_rect_for_index(sprite_index + offset))
    }
}

impl Sprite for DetailedCharacter {
    fn draw(&self, canvas: &mut RenderCanvas, screen_position: SDLPoint, state: u32, frame: u64) -> BoxResult<()> {
        if let Some(state) = CharacterAnimationState::from_u32(state) {
            let screen_rect = SDLRect::from_center(screen_position, 96, 96);
            let (texture, texture_rect) = self.get_texture(&state, frame);
            canvas.copy(texture, texture_rect, screen_rect)?;
        }
        Ok(())
    }
}
