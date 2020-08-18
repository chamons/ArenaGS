use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::TILE_SIZE;
use crate::after_image::{load_image, RenderCanvas, RenderContext};
use crate::atlas::{get_exe_folder, BoxResult, EasyPath};
use crate::clash::ShortInfo;

pub struct CharacterOverlay {
    large_frame: Texture,
    small_frame: Texture,
}

impl CharacterOverlay {
    pub fn init(render_context: &RenderContext) -> BoxResult<CharacterOverlay> {
        let mut small_frame = load_image(
            Path::new(&get_exe_folder()).join("images").join("frames").join("small_frame.png").stringify(),
            render_context,
        )?;
        small_frame.set_alpha_mod(212);
        let mut large_frame = load_image(
            Path::new(&get_exe_folder()).join("images").join("frames").join("large_frame.png").stringify(),
            render_context,
        )?;
        large_frame.set_alpha_mod(212);
        Ok(CharacterOverlay { small_frame, large_frame })
    }

    pub fn draw_character_overlay(&self, canvas: &mut RenderCanvas, ecs: &World, entity: &Entity, screen_position: SDLPoint) -> BoxResult<()> {
        let position = ecs.get_position(entity);
        if position.width == 1 && position.height == 1 {
            let image_rect = SDLRect::new(0, 0, TILE_SIZE, TILE_SIZE);
            let screen_rect = SDLRect::new(screen_position.x() - (TILE_SIZE as i32 / 2), screen_position.y(), TILE_SIZE, TILE_SIZE);
            canvas.copy(&self.small_frame, image_rect, screen_rect)?;
        } else if position.width == 2 && position.height == 2 {
            let image_rect = SDLRect::new(0, 0, TILE_SIZE * 2, TILE_SIZE * 2);
            let screen_rect = SDLRect::new(
                screen_position.x() - TILE_SIZE as i32,
                screen_position.y() - TILE_SIZE as i32,
                TILE_SIZE * 2,
                TILE_SIZE * 2,
            );
            canvas.copy(&self.large_frame, image_rect, screen_rect)?;
        }

        Ok(())
    }
}
