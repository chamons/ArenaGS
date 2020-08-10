use std::path::Path;

use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::components::*;
use super::views::*;
use super::{battle_actions, battle_animations, tick_animations, AnimationComponent, SecondaryAnimation};
use crate::clash::*;

use crate::after_image::{CharacterAnimationState, RenderCanvas, RenderContext, TextRenderer};
use crate::atlas::{get_exe_folder, BoxResult, EasyPath, Point, SizedPoint};
use crate::conductor::{EventStatus, Scene};

pub struct BattleScene<'a> {
    ecs: World,
    views: Vec<Box<dyn View + 'a>>,
}

pub fn add_ui_extension(ecs: &mut World) {
    ecs.register::<RenderComponent>();
    ecs.register::<BattleSceneStateComponent>();
    ecs.register::<MousePositionComponent>();
    ecs.register::<AnimationComponent>();

    ecs.subscribe(BattleScene::on_event);
    ecs.subscribe(super::battle_animations::move_event);

    ecs.insert(BattleSceneStateComponent::init());
    ecs.insert(MousePositionComponent::init());
}

impl<'a> BattleScene<'a> {
    pub fn init(render_context: &RenderContext, text: &'a TextRenderer) -> BoxResult<BattleScene<'a>> {
        let mut ecs = create_world();
        add_ui_extension(&mut ecs);

        ecs.create_entity()
            .with(RenderComponent::init_with_char_state(
                SpriteKinds::MaleBrownHairBlueBody,
                CharacterAnimationState::Idle,
            ))
            .with(PositionComponent::init(SizedPoint::init(4, 4)))
            .with(CharacterInfoComponent::init(Character::init()))
            .with(PlayerComponent::init())
            .with(TimeComponent::init(0))
            .with(SkillResourceComponent::init(&[(AmmoKind::Bullets, 6)]).with_focus(1.0))
            .with(SkillsComponent::init(&["Dash", "Fire Bolt", "Slash", "Strong Shot", "Delayed Blast"]))
            .build();

        ecs.create_entity()
            .with(RenderComponent::init(SpriteKinds::MonsterBirdBrown))
            .with(PositionComponent::init(SizedPoint::init_multi(5, 5, 2, 2)))
            .with(CharacterInfoComponent::init(Character::init()))
            .with(BehaviorComponent::init(BehaviorKind::Random))
            .with(TimeComponent::init(0))
            .build();

        let map_data_path = Path::new(&get_exe_folder()).join("maps").join("beach").join("map1.dat");
        let map_data_path = map_data_path.stringify();
        ecs.insert(MapComponent::init(Map::init(map_data_path)?));

        ecs.create_entity()
            .with(RenderComponent::init_with_order(SpriteKinds::BeachBackground, RenderOrder::Background))
            .build();

        let mut views: Vec<Box<dyn View>> = vec![
            Box::from(MapView::init(render_context)?),
            Box::from(InfoBarView::init(SDLPoint::new(780, 20), text)?),
            Box::from(LogView::init(SDLPoint::new(780, 450), text)?),
            Box::from(SkillBarView::init(
                render_context,
                &ecs,
                SDLPoint::new(137, 40 + super::views::MAP_CORNER_Y as i32 + super::views::TILE_SIZE as i32 * 13i32),
                text,
            )?),
        ];

        if cfg!(debug_assertions) {
            views.push(Box::from(DebugView::init(SDLPoint::new(20, 20), text)?));
        }

        Ok(BattleScene { ecs, views })
    }

    fn on_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
        match kind {
            EventKind::Bolt() => battle_animations::begin_ranged_cast_animation(ecs, &target.unwrap()),
            EventKind::Melee() => battle_animations::begin_melee_animation(ecs, &target.unwrap()),
            EventKind::Move(state) => {}
            EventKind::Field() => {}
            #[cfg(test)]
            EventKind::WaitForAnimations() => super::complete_animations(ecs),
        }
    }

    fn handle_default_key(&mut self, keycode: Keycode) -> EventStatus {
        if cfg!(debug_assertions) {
            if keycode == Keycode::F1 {
                battle_actions::set_state(&mut self.ecs, BattleSceneState::Debug(DebugKind::MapOverlay()));
            }
        }

        if let Some(i) = is_keystroke_skill(keycode) {
            if let Some(name) = battle_actions::get_skill_name(&self.ecs, i as usize) {
                battle_actions::select_skill(&mut self.ecs, &name);
            }
        }
        match keycode {
            Keycode::Up => battle_actions::move_action(&mut self.ecs, Direction::North),
            Keycode::Down => battle_actions::move_action(&mut self.ecs, Direction::South),
            Keycode::Left => battle_actions::move_action(&mut self.ecs, Direction::West),
            Keycode::Right => battle_actions::move_action(&mut self.ecs, Direction::East),
            _ => {}
        }
        EventStatus::Continue
    }

    fn handle_target_key(&mut self, keycode: Keycode) -> EventStatus {
        if keycode == Keycode::Escape {
            battle_actions::reset_state(&mut self.ecs)
        }

        // If they select a skill, start a new target session just like
        if let Some(i) = is_keystroke_skill(keycode) {
            if let Some(name) = battle_actions::get_skill_name(&self.ecs, i as usize) {
                battle_actions::select_skill(&mut self.ecs, &name);
            }
        }
        EventStatus::Continue
    }

    fn handle_debug_key(&mut self, kind: DebugKind, keycode: Keycode) -> EventStatus {
        if keycode == Keycode::Escape {
            battle_actions::reset_state(&mut self.ecs);
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
                battle_actions::select_skill(&mut self.ecs, &name)
            }
        }
        EventStatus::Continue
    }

    fn handle_target_mouse(&mut self, x: i32, y: i32, button: MouseButton) -> EventStatus {
        if button == MouseButton::Right {
            battle_actions::reset_state(&mut self.ecs);
            return EventStatus::Continue;
        }

        let target_info = match battle_actions::read_state(&self.ecs) {
            BattleSceneState::Targeting(target_source) => Some(target_source),
            _ => None,
        };

        if let Some(target_source) = target_info {
            if button == MouseButton::Left {
                if let Some(map_position) = screen_to_map_position(x, y) {
                    match target_source {
                        BattleTargetSource::Skill(skill_name) => battle_actions::select_skill_with_target(&mut self.ecs, &skill_name, &map_position),
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
            BattleSceneState::Targeting(_) => self.handle_target_key(keycode),
            BattleSceneState::Debug(kind) => self.handle_debug_key(kind, keycode),
        }
    }

    fn handle_mouse(&mut self, x: i32, y: i32, button: Option<MouseButton>) -> EventStatus {
        self.ecs.write_resource::<MousePositionComponent>().position = Point::init(x as u32, y as u32);

        if let Some(button) = button {
            if button == MouseButton::Middle {
                let hit = self.views.iter().filter_map(|v| v.hit_test(&self.ecs, x, y)).next();
                match &hit {
                    Some(HitTestResult::Skill(name)) => self.ecs.log(&name),
                    Some(HitTestResult::Tile(position)) => self.ecs.log(&position.to_string()),
                    _ => {}
                }
            }

            let state = battle_actions::read_state(&self.ecs);
            match state {
                BattleSceneState::Default() => self.handle_default_mouse(x, y, button),
                BattleSceneState::Targeting(_) => self.handle_target_mouse(x, y, button),
                BattleSceneState::Debug(kind) => self.handle_debug_mouse(kind, x, y, button),
            }
        } else {
            EventStatus::Continue
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
        process_tick_events(&mut self.ecs, frame)?;
        if !battle_actions::has_animations_blocking(&self.ecs) {
            tick_next_action(&mut self.ecs);
        }

        Ok(())
    }
}

pub fn process_tick_events(ecs: &mut World, frame: u64) -> BoxResult<()> {
    ecs.maintain();
    let completed = tick_animations(ecs, frame)?;
    for (entity, secondary) in completed {
        match secondary {
            SecondaryAnimation::StartBolt => super::battle_animations::begin_ranged_bolt_animation(ecs, &entity),
            SecondaryAnimation::None => {}
        }
    }
    tick_delayed_effects(ecs, frame);
    Ok(())
}

fn is_keystroke_skill(keycode: Keycode) -> Option<u32> {
    let name = keycode.name();
    let chars: Vec<char> = name.chars().collect();

    if chars.len() == 1 {
        match chars[0] {
            '0'..='9' => Some(chars[0].to_string().parse().unwrap()),
            _ => None,
        }
    } else {
        None
    }
}
