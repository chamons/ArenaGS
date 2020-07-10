use enum_iterator::IntoEnumIterator;
use specs::prelude::*;

use super::{Animation, AnimationComponent, CharacterInfoComponent, FieldComponent, PlayerComponent, PositionComponent, RenderComponent, RenderOrder};
use crate::clash::{Character, Point};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::BlendMode;

use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;

use super::{SpriteKinds, SpriteLoader};

use crate::after_image::{CharacterAnimationState, RenderCanvas, RenderContext, TextRenderer};
use crate::atlas::BoxResult;
use crate::conductor::{EventStatus, Scene};

const MAP_CORNER_X: u32 = 100;
const MAP_CORNER_Y: u32 = 100;
const TILE_SIZE: u32 = 48;

pub struct BattleScene<'a> {
    ecs: World,
    sprites: SpriteLoader,
    text: &'a TextRenderer<'a>,
}

impl<'a> BattleScene<'a> {
    pub fn init(render_context: &RenderContext, text: &'a TextRenderer<'a>) -> BoxResult<BattleScene<'a>> {
        let mut ecs = World::new();
        ecs.register::<PositionComponent>();
        ecs.register::<RenderComponent>();
        ecs.register::<AnimationComponent>();
        ecs.register::<FieldComponent>();
        ecs.register::<PlayerComponent>();
        ecs.register::<CharacterInfoComponent>();

        let sprites = SpriteLoader::init(render_context)?;

        ecs.create_entity()
            .with(RenderComponent::init_with_char_state(
                SpriteKinds::MaleBrownHairBlueBody,
                CharacterAnimationState::Idle,
            ))
            .with(PositionComponent::init(2, 2))
            .with(AnimationComponent::sprite_state(
                CharacterAnimationState::Bow,
                CharacterAnimationState::Idle,
                0,
                40,
            ))
            .with(CharacterInfoComponent::init(Character::init()))
            .with(PlayerComponent {})
            .build();

        ecs.create_entity()
            .with(RenderComponent::init(SpriteKinds::MonsterBirdBrown))
            .with(PositionComponent::init(5, 5))
            .with(AnimationComponent::movement(Point::init(5, 5), Point::init(7, 7), 0, 120))
            .build();

        ecs.create_entity()
            .with(RenderComponent::init_with_order(SpriteKinds::BeachBackground, RenderOrder::Background))
            .build();

        ecs.create_entity()
            .with(PositionComponent::init(4, 7))
            .with(FieldComponent::init(255, 0, 0))
            .build();
        ecs.create_entity()
            .with(PositionComponent::init(2, 2))
            .with(FieldComponent::init(0, 0, 255))
            .build();

        Ok(BattleScene { ecs, sprites, text })
    }

    fn draw_grid(&self, canvas: &mut RenderCanvas) -> BoxResult<()> {
        for x in 0..13 {
            for y in 0..13 {
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

    fn render_entities(&self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        let positions = self.ecs.read_storage::<PositionComponent>();
        let renderables = self.ecs.read_storage::<RenderComponent>();
        let animations = self.ecs.read_storage::<AnimationComponent>();

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

    fn render_fields(&self, canvas: &mut RenderCanvas) -> BoxResult<()> {
        let positions = self.ecs.read_storage::<PositionComponent>();
        let fields = self.ecs.read_storage::<FieldComponent>();

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

    fn render_character_info(&self, canvas: &mut RenderCanvas) -> BoxResult<()> {
        self.text.render_text("Hello World", 800, 100, canvas)?;

        Ok(())
    }
}

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

impl<'a> Scene for BattleScene<'a> {
    fn handle_event(&self, event: &sdl2::event::Event) -> EventStatus {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return EventStatus::Quit,
            _ => {}
        }
        EventStatus::Continue
    }

    fn render(&self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 128, 255)));
        canvas.clear();

        self.render_fields(canvas)?;
        self.render_entities(canvas, frame)?;
        self.draw_grid(canvas)?;
        self.render_character_info(canvas)?;

        canvas.present();
        Ok(())
    }

    fn tick(&mut self, frame: u64) -> BoxResult<()> {
        self.ecs.maintain();
        let entities = self.ecs.read_resource::<specs::world::EntitiesRes>();
        let mut animations = self.ecs.write_storage::<AnimationComponent>();
        let mut positions = self.ecs.write_storage::<PositionComponent>();

        // Remove completed animations, applying their change
        let mut completed = vec![];
        for (entity, animation, position) in (&entities, &animations, (&mut positions).maybe()).join() {
            if animation.is_complete(frame) {
                completed.push(entity);
            }
            match &animation.animation {
                Animation::Position { start: _, end } => {
                    if let Some(position) = position {
                        position.x = end.x;
                        position.y = end.y;
                    }
                }
                _ => {}
            }
        }
        for c in completed {
            animations.remove(c);
        }
        Ok(())
    }
}
