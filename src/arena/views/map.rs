use enum_iterator::IntoEnumIterator;
use sdl2::pixels::Color;
use sdl2::render::BlendMode;
use specs::prelude::*;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;

use super::super::components::*;
use super::{HitTestResult, View};

use super::super::SpriteLoader;
use crate::after_image::{RenderCanvas, RenderContext};
use crate::atlas::BoxResult;
use crate::clash::Point;

pub struct MapView {
    sprites: SpriteLoader,
}

pub const MAP_CORNER_X: u32 = 50;
pub const MAP_CORNER_Y: u32 = 50;
pub const TILE_SIZE: u32 = 48;
pub const MAX_MAP_TILES: u32 = 13;

fn get_render_sprite_state(render: &RenderComponent, animation: Option<&AnimationComponent>) -> u32 {
    if let Some(animation) = animation {
        if let Some(state) = animation.current_character_state() {
            return (*state).into();
        }
    }
    render.sprite_state
}

fn get_render_position(position: &PositionComponent, animation: Option<&AnimationComponent>, frame: u64) -> SDLPoint {
    if let Some(animation) = animation {
        if let Some(animation_point) = animation.current_position(frame) {
            return SDLPoint::new(
                ((animation_point.x * TILE_SIZE as f32) + MAP_CORNER_X as f32 + (TILE_SIZE as u32 / 2) as f32) as i32,
                ((animation_point.y * TILE_SIZE as f32) + MAP_CORNER_Y as f32) as i32,
            );
        }
    }
    SDLPoint::new(
        ((position.x * TILE_SIZE as u32) + MAP_CORNER_X + (TILE_SIZE as u32 / 2)) as i32,
        ((position.y * TILE_SIZE as u32) + MAP_CORNER_Y) as i32,
    )
}

impl MapView {
    pub fn init(render_context: &RenderContext) -> BoxResult<MapView> {
        let sprites = SpriteLoader::init(render_context)?;

        Ok(MapView { sprites })
    }

    fn draw_grid(&self, canvas: &mut RenderCanvas) -> BoxResult<()> {
        for x in 0..MAX_MAP_TILES {
            for y in 0..MAX_MAP_TILES {
                canvas.set_draw_color(Color::from((196, 196, 196)));
                canvas.draw_rect(SDLRect::from((
                    (MAP_CORNER_X + x * TILE_SIZE) as i32,
                    (MAP_CORNER_Y + y * TILE_SIZE) as i32,
                    TILE_SIZE as u32,
                    TILE_SIZE as u32,
                )))?;
            }
        }

        Ok(())
    }

    fn render_entities(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        let positions = ecs.read_storage::<PositionComponent>();
        let renderables = ecs.read_storage::<RenderComponent>();
        let animations = ecs.read_storage::<AnimationComponent>();

        // FIXME - Enumerating all renderables 3 times is not ideal, can we do one pass if we get a bunch?
        for order in RenderOrder::into_enum_iter() {
            for (render, position, animation) in (&renderables, (&positions).maybe(), (&animations).maybe()).join() {
                if render.order == order {
                    let id = render.sprite_id;
                    let sprite = &self.sprites.get(id);
                    if let Some(position) = position {
                        let offset = get_render_position(position, animation, frame);
                        let state = get_render_sprite_state(render, animation);
                        sprite.draw(canvas, offset, state, frame)?;
                    } else {
                        sprite.draw(canvas, SDLPoint::new(0, 0), render.sprite_state, frame)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn render_fields(&self, ecs: &World, canvas: &mut RenderCanvas) -> BoxResult<()> {
        let positions = ecs.read_storage::<PositionComponent>();
        let fields = ecs.read_storage::<FieldComponent>();

        for (position, field) in (&positions, &fields).join() {
            let field_rect = SDLRect::new(
                ((position.x * TILE_SIZE as u32) + MAP_CORNER_X + 1) as i32,
                ((position.y * TILE_SIZE as u32) + MAP_CORNER_Y + 1) as i32,
                TILE_SIZE - 2,
                TILE_SIZE - 2,
            );
            canvas.set_draw_color(field.color);
            canvas.set_blend_mode(BlendMode::Blend);
            canvas.fill_rect(field_rect)?;
        }

        Ok(())
    }

    fn screen_to_map_position(&self, x: i32, y: i32) -> Option<Point> {
        // First remove map offset
        let x = x - MAP_CORNER_X as i32;
        let y = y - MAP_CORNER_Y as i32;

        if x < 0 || y < 0 {
            return None;
        }

        // Now divide by grid position
        let x = x as u32 / TILE_SIZE;
        let y = y as u32 / TILE_SIZE;

        // Don't go off map
        if x >= MAX_MAP_TILES || y >= MAX_MAP_TILES {
            return None;
        }
        Some(Point::init(x, y))
    }
}

impl View for MapView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        let state = &ecs.read_resource::<BattleSceneStateComponent>().state;

        self.render_entities(ecs, canvas, frame)?;
        if state.is_targeting() {
            self.draw_grid(canvas)?;
        }
        self.render_fields(ecs, canvas)?;
        Ok(())
    }

    fn hit_test(&self, _: &World, x: i32, y: i32) -> HitTestResult {
        if let Some(point) = self.screen_to_map_position(x, y) {
            HitTestResult::Tile(point)
        } else {
            HitTestResult::None
        }
    }
}
