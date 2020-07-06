use std::collections::HashMap;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;

use super::super::{BattleState, CharacterStyle};

use crate::after_image::{Background, CharacterAnimationState, DetailedCharacterSprite, RenderContext, SpriteDeepFolderDescription};
use crate::atlas::{BoxResult, Point};
use crate::conductor::{EventStatus, Scene};

pub struct BattleScene {
    state: BattleState,
    sprite: HashMap<u32, DetailedCharacterSprite>,
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

    fn draw_background(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> BoxResult<()> {
        let map_corner = SDLPoint::new(200, 200);
        self.background.draw(canvas, map_corner)?;

        Ok(())
    }

    fn draw_field(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, frame: u64) -> BoxResult<()> {
        let map_corner = Point::init(32, 32);

        for x in 0..12 {
            for y in 0..12 {
                canvas.set_draw_color(Color::from((196, 196, 196)));
                canvas.draw_rect(SDLRect::from((map_corner.x as i32 + x * 48, map_corner.y as i32 + y * 48, 48, 48)))?;
            }
        }

        for c in &self.state.party {
            let sprite = &self.sprite[&c.id];
            let offset = SDLPoint::new(((c.position.x * 48) + map_corner.x + 24) as i32, ((c.position.y * 48) + map_corner.y) as i32);
            sprite.draw(canvas, offset, CharacterAnimationState::Idle, frame)?;
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

        self.draw_background(canvas)?;
        self.draw_field(canvas, frame)?;

        canvas.present();

        Ok(())
    }
}
