use enum_iterator::IntoEnumIterator;
use sdl2::pixels::Color;
use specs::prelude::*;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;

use super::super::components::*;
use super::super::{battle_actions, AnimationComponent, SpriteLoader};
use super::view_components::{Frame, FrameKind};
use super::{CharacterOverlay, HitTestResult, View};

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::*;

pub struct MapView {
    sprites: SpriteLoader,
    overlay: CharacterOverlay,
    frame: Frame,
}

pub const MAP_CORNER_X: u32 = 50;
pub const MAP_CORNER_Y: u32 = 50;
pub const TILE_SIZE: u32 = 48;

fn get_render_frame(animation: Option<&AnimationComponent>, frame: u64) -> u64 {
    if let Some(animation_component) = animation {
        frame - animation_component.animation.beginning
    } else {
        frame
    }
}

fn get_render_sprite_state(render: &RenderInfo, animation: Option<&AnimationComponent>) -> u32 {
    if let Some(animation_component) = animation {
        if let Some(state) = animation_component.animation.current_character_state() {
            return (*state).into();
        }
    }
    render.sprite_state
}

fn get_render_position(position: &PositionComponent, animation: Option<&AnimationComponent>, frame: u64) -> SDLPoint {
    let position = position.position;
    let width = position.width;
    if let Some(animation_component) = animation {
        if let Some(animation_point) = animation_component.animation.current_position(frame) {
            return SDLPoint::new(
                ((animation_point.x * TILE_SIZE as f32) + MAP_CORNER_X as f32 + ((width * TILE_SIZE) as u32 / 2) as f32) as i32,
                ((animation_point.y * TILE_SIZE as f32) + MAP_CORNER_Y as f32) as i32,
            );
        }
    }
    SDLPoint::new(
        ((position.origin.x * TILE_SIZE as u32) + MAP_CORNER_X + ((width * TILE_SIZE) as u32 / 2)) as i32,
        ((position.origin.y * TILE_SIZE as u32) + MAP_CORNER_Y) as i32,
    )
}

impl MapView {
    pub fn init(render_context: &RenderContext) -> BoxResult<MapView> {
        let sprites = SpriteLoader::init(render_context)?;
        let overlay = CharacterOverlay::init(render_context)?;
        Ok(MapView {
            sprites,
            overlay,
            frame: Frame::init(SDLPoint::new(0, 0), render_context, FrameKind::Map)?,
        })
    }

    fn draw_grid(&self, canvas: &mut RenderCanvas) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((196, 196, 196)));
        for x in 0..MAX_MAP_TILES {
            for y in 0..MAX_MAP_TILES {
                canvas.draw_rect(screen_rect_for_map_grid(x, y))?;
            }
        }

        Ok(())
    }

    fn render_entities(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        let entities = ecs.read_resource::<specs::world::EntitiesRes>();
        let positions = ecs.read_storage::<PositionComponent>();
        let renderables = ecs.read_storage::<RenderComponent>();
        let animations = ecs.read_storage::<AnimationComponent>();
        let is_characters = ecs.read_storage::<IsCharacterComponent>();
        let skip_renders = ecs.read_storage::<SkipRenderComponent>();

        // FIXME - Enumerating all renderables 3 times is not ideal, can we do one pass if we get a bunch?
        for order in RenderOrder::into_enum_iter() {
            for (entity, render, position, animation, is_character, skip_render) in (
                &entities,
                &renderables,
                (&positions).maybe(),
                (&animations).maybe(),
                (&is_characters).maybe(),
                (&skip_renders).maybe(),
            )
                .join()
            {
                let render = &render.render;
                if render.order == order && skip_render.is_none() {
                    let id = render.sprite_id;
                    let sprite = &self.sprites.get(id);
                    // Animations have a relative frame that starts at 0 (start of animation) and
                    // lasts through duration. We need to use the "real" frame for render position
                    // and the relative "render_frame" for the rest if we have it
                    let render_frame = get_render_frame(animation, frame);
                    if let Some(position) = position {
                        let offset = get_render_position(position, animation, frame);
                        let state = get_render_sprite_state(&render, animation);
                        sprite.draw(canvas, offset, state, render_frame)?;
                        if is_character.is_some() {
                            self.overlay.draw_character_overlay(canvas, ecs, entity, offset)?;
                        }
                    } else {
                        sprite.draw(canvas, SDLPoint::new(0, 0), render.sprite_state, render_frame)?;
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
            for (p, (r, g, b, a)) in &field.fields {
                if let Some(p) = p {
                    self.draw_overlay_tile(canvas, p, Color::from((*r, *g, *b, *a)))?;
                } else {
                    for p in position.position.all_positions() {
                        self.draw_overlay_tile(canvas, &p, Color::from((*r, *g, *b, *a)))?;
                    }
                }
            }
        }

        Ok(())
    }

    fn render_cursor(&self, canvas: &mut RenderCanvas, ecs: &World) -> BoxResult<()> {
        let mouse = ecs.get_mouse_position();
        if let Some(map_position) = screen_to_map_position(mouse.x as i32, mouse.y as i32) {
            if let Some(skill) = get_target_skill(ecs) {
                let player = find_player(&ecs);
                let player_position = ecs.get_position(player);

                let color = if is_good_target(ecs, player, skill, map_position) {
                    Color::from((196, 196, 0, 140))
                } else {
                    Color::from((196, 0, 0, 140))
                };

                if skill.show_trail() {
                    if let Some(points) = player_position.line_to(map_position) {
                        self.draw_line(canvas, &points, color)?;
                    } else {
                        self.draw_overlay_tile(canvas, &map_position, color)?;
                    }
                } else {
                    self.draw_overlay_tile(canvas, &map_position, color)?;
                }
            }
        }

        Ok(())
    }

    fn render_targetting_range(&self, canvas: &mut RenderCanvas, ecs: &World) -> BoxResult<()> {
        if let Some(skill) = get_target_skill(ecs) {
            if let Some(secondary_skill_range) = skill_secondary_range(skill) {
                self.draw_secondary_range_for_mouse(canvas, ecs, skill, secondary_skill_range)?;
            } else {
                self.draw_skill_range_overlap(canvas, ecs, skill)?;
            }
        }
        Ok(())
    }

    fn draw_secondary_range_for_mouse(&self, canvas: &mut RenderCanvas, ecs: &World, skill: &SkillInfo, secondary_skill_range: u32) -> BoxResult<()> {
        let mouse = ecs.get_mouse_position();
        if let Some(mouse_position) = screen_to_map_position(mouse.x as i32, mouse.y as i32) {
            let player = find_player(&ecs);
            if is_good_target(ecs, player, skill, mouse_position) {
                for x in 0..MAX_MAP_TILES {
                    for y in 0..MAX_MAP_TILES {
                        let map_position = Point::init(x, y);
                        if let Some(map_distance) = mouse_position.distance_to(map_position) {
                            if map_distance <= secondary_skill_range {
                                self.draw_overlay_tile(canvas, &map_position, Color::from((196, 196, 0, 60)))?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn draw_skill_range_overlap(&self, canvas: &mut RenderCanvas, ecs: &World, skill: &SkillInfo) -> BoxResult<()> {
        let player = find_player(&ecs);
        for x in 0..MAX_MAP_TILES {
            for y in 0..MAX_MAP_TILES {
                let map_position = Point::init(x, y);
                if in_possible_skill_range(ecs, player, skill, map_position) {
                    self.draw_overlay_tile(canvas, &map_position, Color::from((196, 196, 0, 60)))?;
                }
            }
        }
        Ok(())
    }

    fn draw_line(&self, canvas: &mut RenderCanvas, points: &[Point], color: Color) -> BoxResult<()> {
        for p in points.iter() {
            self.draw_overlay_tile(canvas, &p, color)?;
        }
        Ok(())
    }

    fn draw_overlay_tile(&self, canvas: &mut RenderCanvas, position: &Point, color: Color) -> BoxResult<()> {
        let grid_rect = screen_rect_for_map_grid(position.x, position.y);
        let field_rect = SDLRect::new(grid_rect.x() + 1, grid_rect.y() + 1, grid_rect.width() - 2, grid_rect.height() - 2);
        canvas.set_draw_color(color);
        canvas.fill_rect(field_rect)?;
        Ok(())
    }
}

impl View for MapView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        self.render_entities(ecs, canvas, frame)?;
        if should_draw_grid(ecs) {
            self.draw_grid(canvas)?;
        }
        if should_draw_cursor(ecs) {
            self.render_cursor(canvas, ecs)?;
            self.render_targetting_range(canvas, ecs)?;
        }
        self.render_fields(ecs, canvas)?;

        self.frame.render(ecs, canvas, frame)?;
        Ok(())
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        if let Some(map_position) = screen_to_map_position(x, y) {
            match element_at_location(ecs, &map_position) {
                MapHitTestResult::Enemy => Some(HitTestResult::Enemy(map_position)),
                MapHitTestResult::Field => Some(HitTestResult::Field(map_position)),
                MapHitTestResult::Orb => Some(HitTestResult::Orb(map_position)),
                MapHitTestResult::None => Some(HitTestResult::Tile(map_position)),
            }
        } else {
            None
        }
    }
}

pub fn screen_rect_for_map_grid(x: u32, y: u32) -> SDLRect {
    SDLRect::from((
        (MAP_CORNER_X + x * TILE_SIZE) as i32,
        (MAP_CORNER_Y + y * TILE_SIZE) as i32,
        TILE_SIZE as u32,
        TILE_SIZE as u32,
    ))
}

pub fn screen_to_map_position(x: i32, y: i32) -> Option<Point> {
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

fn should_draw_grid(ecs: &World) -> bool {
    let state = battle_actions::read_action_state(ecs);
    if let BattleSceneState::Debug(kind) = state {
        if kind.is_map_overlay() {
            return true;
        }
    }

    false
}

fn should_draw_cursor(ecs: &World) -> bool {
    let state = battle_actions::read_action_state(ecs);
    matches!(state, BattleSceneState::Targeting(_))
}

fn get_target_skill(ecs: &World) -> Option<&SkillInfo> {
    let state = battle_actions::read_action_state(ecs);
    match state {
        BattleSceneState::Targeting(source) => match source {
            BattleTargetSource::Skill(name) => Some(get_skill(&name)),
        },
        _ => None,
    }
}
