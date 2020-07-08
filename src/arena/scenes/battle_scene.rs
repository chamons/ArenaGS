use std::collections::HashMap;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;

use super::super::{BattleState, CharacterStyle};

use crate::after_image::{
    Background, CharacterAnimationState, DetailedCharacterSprite, LargeEnemySprite, RenderContext, Sprite, SpriteFolderDescription, SpriteState,
};
use crate::atlas::{BoxResult, Point};
use crate::conductor::{EventStatus, Scene};

pub struct BattleScene {
    state: BattleState,
    sprite: HashMap<u32, Box<dyn Sprite>>,
    background: Background,
}

impl BattleScene {
    pub fn init(render_context: &RenderContext, state: BattleState) -> BoxResult<BattleScene> {
        Ok(BattleScene {
            sprite: BattleScene::load_sprites(&render_context, &state)?,
            state,
            background: Background::init("beach", render_context)?,
        })
    }

    fn load_sprites(render_context: &RenderContext, state: &BattleState) -> BoxResult<HashMap<u32, Box<dyn Sprite>>> {
        let folder = Path::new("images");

        let mut sprites: HashMap<u32, Box<dyn Sprite>> = HashMap::new();
        for character in &state.party {
            let (set, character_index) = BattleScene::sprite_index(&character.style);
            let sprite = DetailedCharacterSprite::init(render_context, &SpriteFolderDescription::init(&folder, set, character_index))?;
            sprites.insert(character.id, Box::from(sprite));
        }

        let (_, file_name) = BattleScene::sprite_index(&state.enemy.style);
        let sprite = LargeEnemySprite::init(render_context, &SpriteFolderDescription::init_without_set(&folder, file_name))?;
        sprites.insert(state.enemy.id, Box::from(sprite));
        Ok(sprites)
    }

    fn sprite_index(style: &CharacterStyle) -> (&'static str, &'static str) {
        match style {
            CharacterStyle::MaleBrownHairBlueBody => ("1", "1"),
            CharacterStyle::MaleBlueHairRedBody => ("1", "2"),
            CharacterStyle::MonsterBirdBrown => ("", "$monster_bird1"),
            CharacterStyle::MonsterBirdBlue => ("", "$monster_bird2"),
            CharacterStyle::MonsterBirdRed => ("", "$monster_bird3"),
        }
    }

    fn draw_background(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> BoxResult<()> {
        self.background.draw(canvas)?;

        Ok(())
    }

    const MAP_CORNER: Point = Point::init(100, 100);

    fn draw_field(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, frame: u64) -> BoxResult<()> {
        for x in 0..13 {
            for y in 0..13 {
                canvas.set_draw_color(Color::from((196, 196, 196)));
                canvas.draw_rect(SDLRect::from((
                    BattleScene::MAP_CORNER.x as i32 + x * 48,
                    BattleScene::MAP_CORNER.y as i32 + y * 48,
                    48,
                    48,
                )))?;
            }
        }

        for c in &self.state.party {
            self.draw_character(c.id, &c.position, SpriteState::DetailedCharacter(CharacterAnimationState::Idle), canvas, frame)?;
        }

        self.draw_character(self.state.enemy.id, &self.state.enemy.position, SpriteState::LargeEnemy(), canvas, frame)?;

        Ok(())
    }

    fn draw_character(
        &self,
        id: u32,
        position: &Point,
        state: SpriteState,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        frame: u64,
    ) -> BoxResult<()> {
        let sprite = &self.sprite[&id];
        let offset = SDLPoint::new(
            ((position.x * 48) + BattleScene::MAP_CORNER.x + 24) as i32,
            ((position.y * 48) + BattleScene::MAP_CORNER.y) as i32,
        );
        sprite.draw(canvas, offset, state, frame)?;
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

        self.draw_background(canvas)?;
        self.draw_field(canvas, frame)?;

        canvas.present();

        Ok(())
    }
}
