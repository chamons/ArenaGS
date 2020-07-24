use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::BlendMode;
use specs::prelude::*;

use super::{screen_rect_for_map_grid, View};
use crate::after_image::{FontColor, FontSize, RenderCanvas, TextRenderer};
use crate::arena::components::*;
use crate::arena::read_state;
use crate::atlas::BoxResult;
use crate::clash::{MapComponent, Point};

use crate::clash::MAX_MAP_TILES;

pub struct DebugView<'a> {
    position: SDLPoint,
    text: &'a TextRenderer<'a>,
}

impl<'a> DebugView<'a> {
    pub fn init(position: SDLPoint, text: &'a TextRenderer<'a>) -> BoxResult<DebugView> {
        Ok(DebugView { position, text })
    }
}

impl<'a> View for DebugView<'a> {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        if let BattleSceneState::Debug(kind) = read_state(&ecs) {
            let state = format!("Debug: {}", kind.to_string());
            self.text
                .render_text(&state, self.position.x, self.position.y, canvas, FontSize::Small, FontColor::Red)?;

            match kind {
                DebugKind::MapOverlay() => {
                    let map = &ecs.read_resource::<MapComponent>().map;

                    canvas.set_blend_mode(BlendMode::Blend);
                    for x in 0..MAX_MAP_TILES {
                        for y in 0..MAX_MAP_TILES {
                            let grid_rect = screen_rect_for_map_grid(x, y);
                            let field_rect = SDLRect::new(grid_rect.x() + 1, grid_rect.y() + 1, grid_rect.width() - 2, grid_rect.height() - 2);

                            if map.is_walkable(&Point::init(x, y)) {
                                canvas.set_draw_color(Color::from((0, 0, 196, 196)));
                            } else {
                                canvas.set_draw_color(Color::from((196, 0, 0, 196)));
                            }
                            canvas.fill_rect(field_rect)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
