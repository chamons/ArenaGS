use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::view_components::LifeBar;
use super::TILE_SIZE;
use crate::after_image::{IconCache, IconLoader, RenderCanvas, RenderContext};
use crate::atlas::BoxResult;
use crate::clash::{ShortInfo, StatusInfo, StatusKind};

pub struct CharacterOverlay {
    cache: IconCache,
    lifebar: LifeBar,
}

enum OverlayStatus {
    Burning,
    Frozen,
    Static,
    Aimed,
    Armored,
    Regen,
}

impl OverlayStatus {
    fn get_file_name(&self) -> &'static str {
        match self {
            OverlayStatus::Burning => "fire.png",
            OverlayStatus::Frozen => "ice.png",
            OverlayStatus::Static => "shock.png",
            OverlayStatus::Aimed => "aimed.png",
            OverlayStatus::Armored => "armor.png",
            OverlayStatus::Regen => "regen.png",
        }
    }
}

impl CharacterOverlay {
    pub fn init(render_context: &RenderContext) -> BoxResult<CharacterOverlay> {
        Ok(CharacterOverlay {
            cache: IconCache::init_with_alpha(
                render_context,
                IconLoader::init_overlay_icons(),
                &[
                    "small_frame.png",
                    "large_frame.png",
                    "fire.png",
                    "ice.png",
                    "shock.png",
                    "regen.png",
                    "aimed.png",
                    "armor.png",
                ],
                Some(212),
            )?,
            lifebar: LifeBar::init(render_context)?,
        })
    }

    fn get_overlay_statuses(&self, ecs: &World, entity: &Entity) -> Vec<OverlayStatus> {
        let mut status = vec![];
        let temperature = ecs.get_temperature(entity);

        if temperature.is_burning() {
            status.push(OverlayStatus::Burning);
        }
        if temperature.is_freezing() {
            status.push(OverlayStatus::Frozen);
        }
        if ecs.has_status(entity, StatusKind::StaticCharge) {
            status.push(OverlayStatus::Static);
        }
        if ecs.has_status(entity, StatusKind::Armored) {
            status.push(OverlayStatus::Armored);
        }
        if ecs.has_status(entity, StatusKind::Regen) {
            status.push(OverlayStatus::Regen);
        }
        if ecs.has_status(entity, StatusKind::Aimed) {
            status.push(OverlayStatus::Aimed);
        }

        status
    }

    pub fn draw_character_overlay(&self, canvas: &mut RenderCanvas, ecs: &World, entity: &Entity, screen_position: SDLPoint) -> BoxResult<()> {
        let size = {
            let position = ecs.get_position(entity);
            if position.width == 1 && position.height == 1 {
                1
            } else if position.width == 2 && position.height == 2 {
                2
            } else {
                panic!();
            }
        };

        let life_size = {
            match size {
                1 => TILE_SIZE - 5,
                2 => 2 * (TILE_SIZE - 5),
                _ => panic!("Unknown lifebar size"),
            }
        };
        let lifebar_rect = SDLRect::new(
            screen_position.x() - (life_size as i32 / 2) + 2,
            screen_position.y() + ((4 * TILE_SIZE as i32) / 5) + 2,
            life_size - 4,
            6,
        );

        let defenses = ecs.get_defenses(entity);
        let health = defenses.health as f64 / defenses.max_health as f64;
        let absorb = f64::min(defenses.absorb as f64 / defenses.max_health as f64, 1.0);
        self.lifebar.render(lifebar_rect, canvas, health, absorb)?;

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 128));
        for (i, status) in self.get_overlay_statuses(ecs, entity).iter().enumerate().take(if size == 1 { 2 } else { 4 }) {
            let offset = {
                match size {
                    1 => SDLPoint::new(-17, 22),
                    2 => SDLPoint::new(-38, 22),
                    _ => panic!("Unknown overlay width"),
                }
            };
            let status_start = SDLPoint::new(screen_position.x() + offset.x() + (i as i32 * 18), screen_position.y() + offset.y());
            canvas.fill_rect(SDLRect::new(status_start.x(), status_start.y(), 17, 17))?;
            canvas.copy(
                &self.cache.get(status.get_file_name()),
                None,
                SDLRect::new(status_start.x(), status_start.y(), 16, 16),
            )?;
        }

        match size {
            1 => self.draw_small_bracket(canvas, screen_position)?,
            2 => self.draw_large_bracket(canvas, screen_position)?,
            _ => panic!("Unknown bracket size"),
        }

        Ok(())
    }

    fn draw_large_bracket(&self, canvas: &mut RenderCanvas, screen_position: SDLPoint) -> BoxResult<()> {
        let image_rect = SDLRect::new(0, 0, TILE_SIZE * 2, TILE_SIZE * 2);
        let screen_rect = SDLRect::new(
            screen_position.x() - TILE_SIZE as i32,
            screen_position.y() - TILE_SIZE as i32,
            TILE_SIZE * 2,
            TILE_SIZE * 2,
        );
        canvas.copy(self.cache.get("large_frame.png"), image_rect, screen_rect)?;
        Ok(())
    }

    fn draw_small_bracket(&self, canvas: &mut RenderCanvas, screen_position: SDLPoint) -> BoxResult<()> {
        let image_rect = SDLRect::new(0, 0, TILE_SIZE, TILE_SIZE);
        let screen_rect = SDLRect::new(screen_position.x() - (TILE_SIZE as i32 / 2), screen_position.y(), TILE_SIZE, TILE_SIZE);
        canvas.copy(self.cache.get("small_frame.png"), image_rect, screen_rect)?;
        Ok(())
    }
}
