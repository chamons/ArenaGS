use specs::prelude::*;

use super::RenderComponent;
use crate::clash::PositionComponent;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;

use super::{SpriteKinds, SpriteLoader};

use crate::after_image::{CharacterAnimationState, RenderContext, SpriteState};
use crate::atlas::BoxResult;
use crate::conductor::{EventStatus, Scene};

const MAP_CORNER_X: u32 = 100;
const MAP_CORNER_Y: u32 = 100;
const TILE_SIZE: u32 = 48;

pub struct BattleScene {
    ecs: World,
    sprites: SpriteLoader,
}

impl BattleScene {
    pub fn init(render_context: &RenderContext) -> BoxResult<BattleScene> {
        let mut ecs = World::new();
        ecs.register::<PositionComponent>();
        ecs.register::<RenderComponent>();

        let sprites = SpriteLoader::init(render_context)?;

        ecs.create_entity()
            .with(RenderComponent::init_with_order(SpriteKinds::BeachBackground, -1))
            .build();

        ecs.create_entity()
            .with(RenderComponent {
                sprite_id: SpriteKinds::MaleBrownHairBlueBody.into(),
                sprite_state: SpriteState::DetailedCharacter(CharacterAnimationState::Idle),
                z_order: 0,
            })
            .with(PositionComponent::init(2, 2))
            .build();

        ecs.create_entity()
            .with(RenderComponent::init(SpriteKinds::MonsterBirdBrown))
            .with(PositionComponent::init(5, 5))
            .build();

        Ok(BattleScene { ecs, sprites })
    }

    fn draw_field_overlay(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> BoxResult<()> {
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

    fn render_entities(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, frame: u64) -> BoxResult<()> {
        let positions = self.ecs.read_storage::<PositionComponent>();
        let renderables = self.ecs.read_storage::<RenderComponent>();

        for (render, position) in (&renderables, (&positions).maybe()).join() {
            let id = render.sprite_id;
            let sprite = &self.sprites.get(id);
            if let Some(position) = position {
                let offset = SDLPoint::new(
                    ((position.x * TILE_SIZE as u32) + MAP_CORNER_X + (TILE_SIZE as u32 / 2)) as i32,
                    ((position.y * TILE_SIZE as u32) + MAP_CORNER_Y) as i32,
                );
                sprite.draw(canvas, offset, &render.sprite_state, frame)?;
            } else {
                sprite.draw(canvas, SDLPoint::new(0, 0), &SpriteState::None(), frame)?;
            }
        }
        Ok(())
    }
}

impl Scene for BattleScene {
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

    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 128, 255)));
        canvas.clear();

        self.render_entities(canvas, frame)?;
        self.draw_field_overlay(canvas)?;

        canvas.present();
        Ok(())
    }

    fn tick(&mut self) -> BoxResult<()> {
        self.ecs.maintain();

        Ok(())
    }
}
