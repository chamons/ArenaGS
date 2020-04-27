use std::collections::HashMap;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;

use super::super::{BattleState, Character, CharacterStyle};

use crate::after_image::{CharacterAnimationState, DetailedCharacterSprite, RenderContext, SpriteDeepFolderDescription};
use crate::atlas::{BoxResult, Point};
use crate::conductor::{EventStatus, Scene};

pub struct BattleScene {
    state: BattleState,
    sprite: HashMap<u32, DetailedCharacterSprite>,
}

impl BattleScene {
    pub fn init(render_context: &RenderContext, state: BattleState) -> BoxResult<BattleScene> {
        Ok(BattleScene {
            sprite: BattleScene::load_sprites(&render_context, &state)?,
            state,
        })
    }

    fn load_sprites(render_context: &RenderContext, state: &BattleState) -> BoxResult<HashMap<u32, DetailedCharacterSprite>> {
        let folder = Path::new("images").join("battle");

        let mut sprites = HashMap::new();
        for character in &state.party {
            let (set, character_index) = BattleScene::sprite_index(&character.style);
            let sprite = DetailedCharacterSprite::init(render_context, &SpriteDeepFolderDescription::init(&folder, set, character_index))?;
            sprites.insert(character.id, sprite);
        }
        Ok(sprites)
    }

    fn sprite_index(style: &CharacterStyle) -> (&'static str, &'static str) {
        match style {
            CharacterStyle::MaleBrownHairBlueBody => ("1", "1"),
            CharacterStyle::MaleBlueHairRedBody => ("1", "2"),
        }
    }

    fn draw_field(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, frame: u64) -> BoxResult<()> {
        let (width, height) = canvas.output_size()?;

        let corner = Point::init(32, 32);

        for x in 0..12 {
            for y in 0..12 {
                canvas.set_draw_color(Color::from((255, 255, 255)));
                canvas.draw_rect(SDLRect::from((corner.x as i32 + x * 48, corner.y as i32 + y * 48, 48, 48)))?;
            }
        }

        for c in &self.state.party {
            let sprite = &self.sprite[&c.id];
            sprite.draw(canvas, SDLPoint::new(0, 0), CharacterAnimationState::Idle, frame)?;
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

        self.draw_field(canvas, frame)?;

        canvas.present();

        Ok(())
    }
}
