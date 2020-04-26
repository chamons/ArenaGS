use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use crate::after_image::{CharacterAnimationState, DetailedCharacterSprite, RenderContext, SpriteDeepFolderDescription};
use crate::atlas::BoxResult;
use crate::conductor::{EventStatus, Scene};

pub struct BattleScene {
    character_one: DetailedCharacterSprite,
    character_two: DetailedCharacterSprite,
}

impl BattleScene {
    pub fn init(render_context: &mut RenderContext) -> BoxResult<BattleScene> {
        let folder = Path::new("images").join("battle");
        let character_one = DetailedCharacterSprite::init(render_context, &SpriteDeepFolderDescription::init(&folder, "1", "1"))?;
        let character_two = DetailedCharacterSprite::init(render_context, &SpriteDeepFolderDescription::init(&folder, "1", "2"))?;
        Ok(BattleScene { character_one, character_two })
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

        let (width, height) = canvas.output_size()?;
        let sprite = Rect::new(0, 0, 96, 96);
        let screen_position = Point::new(0, 0) + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());
        canvas.copy(self.character_one.get_texture(CharacterAnimationState::Idle, frame), sprite, screen_rect)?;

        let screen_position = Point::new(0, 90) + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());
        canvas.copy(self.character_two.get_texture(CharacterAnimationState::Idle, frame), sprite, screen_rect)?;

        canvas.present();

        Ok(())
    }
}
