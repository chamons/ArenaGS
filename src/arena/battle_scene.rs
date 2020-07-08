use std::collections::HashMap;
use std::path::Path;

use enum_iterator::IntoEnumIterator;
use specs::prelude::*;

use super::RenderComponent;
use crate::clash::PositionComponent;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;

use super::SpriteKinds;

use crate::after_image::{Background, CharacterAnimationState, DetailedCharacter, LargeEnemy, RenderContext, Sprite, SpriteFolderDescription, SpriteState};
use crate::atlas::BoxResult;
use crate::conductor::{EventStatus, Scene};

pub struct BattleScene {
    ecs: World,
    sprite_cache: HashMap<u32, Box<dyn Sprite>>,
}

impl BattleScene {
    pub fn init(render_context: &RenderContext) -> BoxResult<BattleScene> {
        let mut ecs = World::new();
        ecs.register::<PositionComponent>();
        ecs.register::<RenderComponent>();

        let sprite_cache = BattleScene::load_sprites(&render_context)?;

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

        Ok(BattleScene { ecs, sprite_cache })
    }

    pub fn run(&mut self) {
        self.ecs.maintain();
    }

    fn load_sprites(render_context: &RenderContext) -> BoxResult<HashMap<u32, Box<dyn Sprite>>> {
        let folder = Path::new("images");

        let mut sprites: HashMap<u32, Box<dyn Sprite>> = HashMap::new();
        for s in SpriteKinds::into_enum_iter() {
            let sprite: Box<dyn Sprite> = match s {
                SpriteKinds::BeachBackground => Box::new(Background::init(render_context, "beach")?),
                SpriteKinds::MaleBrownHairBlueBody => Box::new(DetailedCharacter::init(render_context, &SpriteFolderDescription::init(&folder, "1", "1"))?),
                SpriteKinds::MaleBlueHairRedBody => Box::new(DetailedCharacter::init(render_context, &SpriteFolderDescription::init(&folder, "1", "1"))?),
                SpriteKinds::MonsterBirdBrown => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird1"),
                )?),
                SpriteKinds::MonsterBirdBlue => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird2"),
                )?),
                SpriteKinds::MonsterBirdRed => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird3"),
                )?),
            };
            sprites.insert(s.into(), sprite);
        }
        Ok(sprites)
    }

    const MAP_CORNER_X: u32 = 100;
    const MAP_CORNER_Y: u32 = 100;

    fn draw_field_overlay(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> BoxResult<()> {
        for x in 0..13 {
            for y in 0..13 {
                canvas.set_draw_color(Color::from((196, 196, 196)));
                canvas.draw_rect(SDLRect::from((
                    BattleScene::MAP_CORNER_X as i32 + x * 48,
                    BattleScene::MAP_CORNER_Y as i32 + y * 48,
                    48,
                    48,
                )))?;
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

        let positions = self.ecs.read_storage::<PositionComponent>();
        let renderables = self.ecs.read_storage::<RenderComponent>();

        for (render, position) in (&renderables, (&positions).maybe()).join() {
            let id = render.sprite_id;
            let sprite = &self.sprite_cache[&id];
            if let Some(position) = position {
                let offset = SDLPoint::new(
                    ((position.x * 48) + BattleScene::MAP_CORNER_X + 24) as i32,
                    ((position.y * 48) + BattleScene::MAP_CORNER_Y) as i32,
                );
                sprite.draw(canvas, offset, &render.sprite_state, frame)?;
            } else {
                sprite.draw(canvas, SDLPoint::new(0, 0), &SpriteState::None(), frame)?;
            }
        }

        self.draw_field_overlay(canvas)?;
        canvas.present();

        Ok(())
    }

    fn tick(&mut self) -> BoxResult<()> {
        Ok(())
    }
}
