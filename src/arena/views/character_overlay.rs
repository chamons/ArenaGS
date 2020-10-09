use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::view_components::LifeBar;
use super::TILE_SIZE;
use crate::after_image::{IconCache, IconLoader, RenderCanvas, RenderContext};
use crate::atlas::BoxResult;
use crate::clash::ShortInfo;

pub struct CharacterOverlay {
    cache: IconCache,
    lifebar: LifeBar,
}

impl CharacterOverlay {
    pub fn init(render_context: &RenderContext) -> BoxResult<CharacterOverlay> {
        Ok(CharacterOverlay {
            cache: IconCache::init_with_alpha(
                render_context,
                IconLoader::init_overlay_icons(),
                &["small_frame.png", "large_frame.png", "fire.png", "ice.png"],
                Some(212),
            )?,
            lifebar: LifeBar::init(render_context)?,
        })
    }

    // Collect a list of statuses, draw first N in order
    // Burning
    // Frozen
    // StaticCharge,
    // Aimed,
    // Armored,
    // Flying,
    // Regen,

    pub fn draw_character_overlay(&self, canvas: &mut RenderCanvas, ecs: &World, entity: &Entity, screen_position: SDLPoint) -> BoxResult<()> {
        let position = ecs.get_position(entity);
        if position.width == 1 && position.height == 1 {
            self.draw_small_bracket(canvas, screen_position)?;
        } else if position.width == 2 && position.height == 2 {
            self.draw_large_bracket(canvas, screen_position)?;
        } else {
            panic!();
        }

        let life_size = self.lifebar_size(ecs, entity);
        let lifebar_rect = SDLRect::new(
            screen_position.x() - (life_size as i32 / 2) + 2,
            screen_position.y() + ((4 * TILE_SIZE as i32) / 5) + 2,
            life_size - 4,
            6,
        );

        let defenses = ecs.get_defenses(entity);
        let health = defenses.health as f64 / defenses.max_health as f64;
        self.lifebar.render(lifebar_rect, canvas, health)?;

        let temperature = ecs.get_temperature(entity);
        let image_rect = SDLRect::new(0, 0, 32, 32);
        let offset = self.status_offset(ecs, entity);
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 64));
        if temperature.is_burning() {
            let screen_rect = SDLRect::new(screen_position.x() + offset.x(), screen_position.y() + offset.y(), 32, 32);
            let background_rect = SDLRect::new(screen_rect.x() + 7, screen_rect.y() + 2, 18, 28);
            canvas.fill_rect(background_rect)?;
            canvas.copy(&self.cache.get("fire.png"), image_rect, screen_rect)?;
        } else if temperature.is_freezing() {
            let screen_rect = SDLRect::new(screen_position.x() + offset.x() + 8, screen_position.y() + offset.y() + 6, 20, 20);
            let background_rect = SDLRect::new(screen_rect.x() + 7 - 8, screen_rect.y() + 2 - 6, 22, 28);
            canvas.fill_rect(background_rect)?;
            canvas.copy(&self.cache.get("ice.png"), image_rect, screen_rect)?
        }

        Ok(())
    }

    fn lifebar_size(&self, ecs: &World, entity: &Entity) -> u32 {
        let position = ecs.get_position(entity);
        if position.width == 1 && position.height == 1 {
            TILE_SIZE - 5
        } else if position.width == 2 && position.height == 2 {
            2 * (TILE_SIZE - 5)
        } else {
            panic!();
        }
    }

    fn status_offset(&self, ecs: &World, entity: &Entity) -> SDLPoint {
        let position = ecs.get_position(entity);
        if position.width == 1 && position.height == 1 {
            SDLPoint::new(-28, 16)
        } else if position.width == 2 && position.height == 2 {
            SDLPoint::new(-52, 16)
        } else {
            panic!();
        }
    }

    fn draw_large_bracket(&self, canvas: &mut RenderCanvas, screen_position: SDLPoint) -> BoxResult<()> {
        let image_rect = SDLRect::new(0, 0, TILE_SIZE * 2, TILE_SIZE * 2);
        let screen_rect = SDLRect::new(
            screen_position.x() - TILE_SIZE as i32,
            screen_position.y() - TILE_SIZE as i32,
            TILE_SIZE * 2,
            TILE_SIZE * 2,
        );
        canvas.copy(self.cache.get("small_frame.png"), image_rect, screen_rect)?;
        Ok(())
    }

    fn draw_small_bracket(&self, canvas: &mut RenderCanvas, screen_position: SDLPoint) -> BoxResult<()> {
        let image_rect = SDLRect::new(0, 0, TILE_SIZE, TILE_SIZE);
        let screen_rect = SDLRect::new(screen_position.x() - (TILE_SIZE as i32 / 2), screen_position.y(), TILE_SIZE, TILE_SIZE);
        canvas.copy(self.cache.get("large_frame.png"), image_rect, screen_rect)?;
        Ok(())
    }
}
