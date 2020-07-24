use std::path::Path;

use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::components::*;
use super::views::*;
use super::*;
use crate::clash::*;

use crate::after_image::{CharacterAnimationState, RenderCanvas, RenderContext, TextRenderer};
use crate::atlas::{get_exe_folder, BoxResult, Logger};
use crate::conductor::{EventStatus, Scene};

pub struct BattleScene<'a> {
    ecs: World,
    views: Vec<Box<dyn View + 'a>>,
}

impl<'a> BattleScene<'a> {
    pub fn init(render_context: &RenderContext, text: &'a TextRenderer) -> BoxResult<BattleScene<'a>> {
        let mut ecs = create_world();
        ecs.register::<RenderComponent>();
        ecs.register::<LogComponent>();
        ecs.register::<BattleSceneStateComponent>();

        ecs.insert(FrameComponent::init());
        ecs.insert(LogComponent::init());
        ecs.insert(BattleSceneStateComponent::init());

        ecs.create_entity()
            .with(RenderComponent::init_with_char_state(
                SpriteKinds::MaleBrownHairBlueBody,
                CharacterAnimationState::Idle,
            ))
            .with(PositionComponent::init(4, 4))
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
            .with(PositionComponent::init_multi(5, 5, 2, 2))
            .with(CharacterInfoComponent::init(Character::init()))
            .build();

        let map_data_path = Path::new(&get_exe_folder()).join("maps").join("beach").join("map1.dat");
        let map_data_path = map_data_path.to_str().unwrap();
        ecs.insert(MapComponent::init(Map::init(map_data_path)?));

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

        let mut views: Vec<Box<dyn View>> = vec![
            Box::from(MapView::init(render_context)?),
            Box::from(InfoBarView::init(SDLPoint::new(780, 20), text)?),
            Box::from(LogView::init(SDLPoint::new(780, 450), text)?),
            Box::from(SkillBarView::init(
                render_context,
                SDLPoint::new(137, 40 + super::views::MAP_CORNER_Y as i32 + super::views::TILE_SIZE as i32 * 13i32),
                text,
            )?),
        ];

        if cfg!(debug_assertions) {
            views.push(Box::from(DebugView::init(SDLPoint::new(20, 20), text)?));
        }

        Ok(BattleScene { ecs, views })
    }

    fn handle_default_key(&mut self, keycode: Keycode) -> EventStatus {
        if cfg!(debug_assertions) {
            if keycode == Keycode::F1 {
                set_state(&mut self.ecs, BattleSceneState::Debug(DebugKind::MapOverlay()));
            }
        }

        if let Some(i) = is_keystroke_skill(keycode) {
            // HACK - should get name from model, not test data
            let name = super::views::test_skill_name(i);
            select_skill(&mut self.ecs, &name);
        }
        match keycode {
            Keycode::Up => move_player(&mut self.ecs, Direction::North),
            Keycode::Down => move_player(&mut self.ecs, Direction::South),
            Keycode::Left => move_player(&mut self.ecs, Direction::West),
            Keycode::Right => move_player(&mut self.ecs, Direction::East),
            _ => {}
        }
        EventStatus::Continue
    }

    fn handle_target_key(&mut self, keycode: Keycode) -> EventStatus {
        if keycode == Keycode::Escape {
            reset_state(&mut self.ecs)
        }

        // If they select a skill, start a new target session just like
        if let Some(i) = is_keystroke_skill(keycode) {
            // HACK - should get name from model, not test data
            let name = super::views::test_skill_name(i);
            select_skill(&mut self.ecs, &name);
        }
        EventStatus::Continue
    }

    fn handle_debug_key(&mut self, kind: DebugKind, keycode: Keycode) -> EventStatus {
        if keycode == Keycode::Escape {
            reset_state(&mut self.ecs);
            return EventStatus::Continue;
        }
        if kind.is_map_overlay() {
            if keycode == Keycode::S {
                let map = &self.ecs.read_resource::<MapComponent>().map;
                map.write_to_file().unwrap();
            }
        }
        EventStatus::Continue
    }

    fn handle_default_mouse(&mut self, x: i32, y: i32, button: MouseButton) -> EventStatus {
        let hit = self.views.iter().filter_map(|v| v.hit_test(&self.ecs, x, y)).next();
        if button == MouseButton::Left {
            if let Some(HitTestResult::Skill(name)) = &hit {
                select_skill(&mut self.ecs, &name)
            }
        }
        EventStatus::Continue
    }

    fn handle_target_mouse(&mut self, x: i32, y: i32, button: MouseButton) -> EventStatus {
        if button == MouseButton::Right {
            reset_state(&mut self.ecs);
            return EventStatus::Continue;
        }

        let target_info = match read_state(&self.ecs) {
            BattleSceneState::Targeting(target_source, target_type) => Some((target_source, target_type)),
            _ => None,
        };

        if let Some((target_source, required_type)) = target_info {
            if button == MouseButton::Left {
                let hit = self.views.iter().filter_map(|v| v.hit_test(&self.ecs, x, y)).next();
                let position = match &hit {
                    Some(HitTestResult::Tile(position)) if required_type.is_tile() => Some(position),
                    Some(HitTestResult::Enemy(position)) if required_type.is_enemy() => Some(position),
                    _ => None,
                };
                if let Some(position) = position {
                    match target_source {
                        BattleTargetSource::Skill(skill_name) => select_skill_with_target(&mut self.ecs, &skill_name, position),
                    }
                }
            }
        }

        EventStatus::Continue
    }

    fn handle_debug_mouse(&mut self, kind: DebugKind, x: i32, y: i32, button: MouseButton) -> EventStatus {
        if button == MouseButton::Left {
            if kind.is_map_overlay() {
                if let Some(map_position) = screen_to_map_position(x, y) {
                    let map = &mut self.ecs.write_resource::<MapComponent>().map;
                    map.set_walkable(&map_position, !map.is_walkable(&map_position));
                }
            }
        }
        EventStatus::Continue
    }
}

impl<'a> Scene for BattleScene<'a> {
    fn handle_key(&mut self, keycode: Keycode) -> EventStatus {
        match keycode {
            Keycode::PageUp => self.ecs.log_scroll_back(),
            Keycode::PageDown => self.ecs.log_scroll_forward(),
            _ => {}
        }

        let state = self.ecs.read_resource::<BattleSceneStateComponent>().state.clone();
        match state {
            BattleSceneState::Default() => self.handle_default_key(keycode),
            BattleSceneState::Targeting(_, _) => self.handle_target_key(keycode),
            BattleSceneState::Debug(kind) => self.handle_debug_key(kind, keycode),
        }
    }

    fn handle_mouse(&mut self, x: i32, y: i32, button: MouseButton) -> EventStatus {
        if button == MouseButton::Middle {
            let hit = self.views.iter().filter_map(|v| v.hit_test(&self.ecs, x, y)).next();
            match &hit {
                Some(HitTestResult::Skill(name)) => self.ecs.log(&name),
                Some(HitTestResult::Tile(position)) => self.ecs.log(&position.to_string()),
                _ => {}
            }
        }

        let state = read_state(&self.ecs);
        match state {
            BattleSceneState::Default() => self.handle_default_mouse(x, y, button),
            BattleSceneState::Targeting(_, _) => self.handle_target_mouse(x, y, button),
            BattleSceneState::Debug(kind) => self.handle_debug_mouse(kind, x, y, button),
        }
    }

    fn render(&self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        self.ecs.write_resource::<FrameComponent>().current_frame = frame;

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

fn is_keystroke_skill(keycode: Keycode) -> Option<u32> {
    let name = keycode.name();
    let chars: Vec<char> = name.chars().collect();

    if chars.len() == 1 {
        match chars[0] {
            // HACK - should get name from model, not test data
            '0'..='9' => Some(chars[0].to_string().parse().unwrap()),
            _ => None,
        }
    } else {
        None
    }
}
