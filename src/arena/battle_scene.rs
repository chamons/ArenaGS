use specs::prelude::*;

use super::{tick_animations, AnimationComponent, CharacterInfoComponent, FieldComponent, PlayerComponent, PositionComponent, RenderComponent, RenderOrder};
use crate::clash::{Character, Point};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use sdl2::rect::Point as SDLPoint;

use super::views::{MapView, SkillBarView, View};
use super::SpriteKinds;

use crate::after_image::{CharacterAnimationState, RenderCanvas, RenderContext, TextRenderer};
use crate::atlas::BoxResult;
use crate::conductor::{EventStatus, Scene};

pub struct BattleScene<'a> {
    ecs: World,
    text: &'a TextRenderer<'a>,
    views: Vec<Box<dyn View>>,
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

        let views: Vec<Box<dyn View>> = vec![
            Box::from(MapView::init(render_context)?),
            Box::from(SkillBarView::init(SDLPoint::new(0, 1i32 + super::views::TILE_SIZE as i32 * 13i32))?),
        ];

        Ok(BattleScene { ecs, text, views })
    }

    fn render_character_info(&self, canvas: &mut RenderCanvas) -> BoxResult<()> {
        self.text.render_text("Hello World", 800, 100, canvas)?;

        Ok(())
    }
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

        self.render_character_info(canvas)?;
        for view in self.views.iter() {
            view.render(&self.ecs, canvas, frame)?;
        }

        canvas.present();
        Ok(())
    }

    fn tick(&mut self, frame: u64) -> BoxResult<()> {
        self.ecs.maintain();
        tick_animations(&self.ecs, frame)?;
        Ok(())
    }
}
