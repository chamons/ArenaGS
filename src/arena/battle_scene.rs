use specs::prelude::*;

use super::components::*;
use crate::clash::{Character, Point};

use sdl2::mouse::MouseButton;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use sdl2::rect::Point as SDLPoint;

use super::views::{HitTestResult, InfoBarView, LogComponent, LogView, MapView, SkillBarView, View};

use crate::after_image::{CharacterAnimationState, RenderCanvas, RenderContext, TextRenderer};
use crate::atlas::{BoxResult, Logger};
use crate::conductor::{EventStatus, Scene};

pub struct BattleScene<'a> {
    ecs: World,
    views: Vec<Box<dyn View + 'a>>,
}

impl<'a> BattleScene<'a> {
    pub fn init(render_context: &RenderContext, text: &'a TextRenderer) -> BoxResult<BattleScene<'a>> {
        let mut ecs = World::new();
        ecs.register::<PositionComponent>();
        ecs.register::<RenderComponent>();
        ecs.register::<AnimationComponent>();
        ecs.register::<FieldComponent>();
        ecs.register::<PlayerComponent>();
        ecs.register::<CharacterInfoComponent>();
        ecs.register::<LogComponent>();
        ecs.register::<BattleSceneStateComponent>();

        ecs.insert(LogComponent::init());
        ecs.insert(BattleSceneStateComponent::init());

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
            Box::from(InfoBarView::init(SDLPoint::new(780, 20), text)?),
            Box::from(LogView::init(SDLPoint::new(780, 450), text)?),
            Box::from(SkillBarView::init(
                render_context,
                SDLPoint::new(137, 40 + super::views::MAP_CORNER_Y as i32 + super::views::TILE_SIZE as i32 * 13i32),
                text,
            )?),
        ];

        Ok(BattleScene { ecs, views })
    }
    fn handle_default_key(&mut self, keycode: &Keycode) -> EventStatus {
        let name = keycode.name();
        let chars: Vec<char> = name.chars().collect();
        if chars.len() == 1 {
            match chars[0] {
                '0'..='9' => self.ecs.log(&chars[0].to_string()),
                _ => {}
            }
        }
        EventStatus::Continue
    }
}

impl<'a> Scene for BattleScene<'a> {
    fn handle_key(&mut self, keycode: &Keycode) -> EventStatus {
        match keycode {
            Keycode::PageUp => self.ecs.log_scroll_back(),
            Keycode::PageDown => self.ecs.log_scroll_forward(),
            _ => {}
        }

        let state = self.ecs.read_resource::<BattleSceneStateComponent>().state.clone();
        match state {
            BattleSceneState::Default() => self.handle_default_key(keycode),
            BattleSceneState::Targeting(_) => EventStatus::Continue,
        }
    }

    fn handle_mouse(&mut self, x: i32, y: i32, button: &MouseButton) -> EventStatus {
        if *button == MouseButton::Middle {
            for view in self.views.iter() {
                match view.hit_test(&self.ecs, x, y) {
                    HitTestResult::Skill(name) => self.ecs.log(&name),
                    HitTestResult::Tile(position) => self.ecs.log(&position.to_string()),
                    _ => {}
                }
            }
        }
        EventStatus::Continue
    }

    fn render(&self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 128, 255)));
        canvas.clear();

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

pub fn tick_animations(ecs: &World, frame: u64) -> BoxResult<()> {
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let mut animations = ecs.write_storage::<AnimationComponent>();
    let mut positions = ecs.write_storage::<PositionComponent>();

    // Remove completed animations, applying their change
    let mut completed = vec![];
    for (entity, animation, position) in (&entities, &animations, (&mut positions).maybe()).join() {
        if animation.is_complete(frame) {
            completed.push(entity);
        }
        if let Animation::Position { start: _, end } = &animation.animation {
            if let Some(position) = position {
                position.x = end.x;
                position.y = end.y;
            }
        }
    }
    for c in completed {
        animations.remove(c);
    }

    Ok(())
}
