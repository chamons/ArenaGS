use std::rc::Rc;

use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::components::*;
use super::views::*;
use super::{battle_actions, complete_animations, tick_animations};
use crate::clash::*;

use super::saveload;
use crate::after_image::{RenderCanvas, RenderContextHolder, TextRenderer};
use crate::atlas::{BoxResult, EasyMutECS, Point};
use crate::conductor::Scene;

pub struct BattleScene {
    ecs: World,
    views: Vec<Box<dyn View>>,
}

impl BattleScene {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>) -> BoxResult<BattleScene> {
        let ecs = saveload::new_world().unwrap();
        {
            ecs.write_storage::<StatusComponent>().grab_mut(find_player(&ecs)).status.add_trait("Fire Ammo");
        }

        let render_context = &render_context_holder.borrow();
        let mut views: Vec<Box<dyn View>> = vec![
            Box::from(MapView::init(&render_context)?),
            Box::from(InfoBarView::init(SDLPoint::new(780, 20), Rc::clone(&text_renderer))?),
            Box::from(LogView::init(SDLPoint::new(780, 450), Rc::clone(&text_renderer))?),
            Box::from(SkillBarView::init(
                render_context,
                &ecs,
                SDLPoint::new(137, 40 + super::views::MAP_CORNER_Y as i32 + super::views::TILE_SIZE as i32 * 13i32),
                Rc::clone(&text_renderer),
            )?),
            Box::from(StatusBarView::init(&render_context, SDLPoint::new(20, 20), &ecs)?),
        ];

        if cfg!(debug_assertions) {
            views.push(Box::from(DebugView::init(SDLPoint::new(20, 20), Rc::clone(&text_renderer))?));
        }

        Ok(BattleScene { ecs, views })
    }

    fn handle_default_key(&mut self, keycode: Keycode) {
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
            Keycode::S => saveload::save(&mut self.ecs),
            Keycode::L => {
                if let Ok(new_world) = saveload::load() {
                    self.ecs = new_world;
                }
            }
            _ => {}
        }
    }

    fn handle_target_key(&mut self, keycode: Keycode) {
        if keycode == Keycode::Escape {
            battle_actions::reset_state(&mut self.ecs)
        }

        // If they select a skill, start a new target session just like
        if let Some(i) = is_keystroke_skill(keycode) {
            if let Some(name) = battle_actions::get_skill_name(&self.ecs, i as usize) {
                battle_actions::select_skill(&mut self.ecs, &name);
            }
        }
    }

    fn handle_debug_key(&mut self, kind: DebugKind, keycode: Keycode) {
        if keycode == Keycode::Escape {
            battle_actions::reset_state(&mut self.ecs);
            return;
        }
        if kind.is_map_overlay() {
            if keycode == Keycode::S {
                let map = &self.ecs.read_resource::<MapComponent>().map;
                map.write_to_file().unwrap();
            }
        }
    }

    fn handle_default_mouse(&mut self, x: i32, y: i32, button: MouseButton) {
        let hit = self.views.iter().filter_map(|v| v.hit_test(&self.ecs, x, y)).next();
        if button == MouseButton::Left {
            if let Some(HitTestResult::Skill(name)) = &hit {
                battle_actions::select_skill(&mut self.ecs, &name)
            }
        }
    }

    fn handle_target_mouse(&mut self, x: i32, y: i32, button: MouseButton) {
        if button == MouseButton::Right {
            battle_actions::reset_state(&mut self.ecs);
            return;
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
    }

    fn handle_debug_mouse(&mut self, kind: DebugKind, x: i32, y: i32, button: MouseButton) {
        if button == MouseButton::Left {
            if kind.is_map_overlay() {
                if let Some(map_position) = screen_to_map_position(x, y) {
                    let map = &mut self.ecs.write_resource::<MapComponent>().map;
                    map.set_walkable(&map_position, !map.is_walkable(&map_position));
                }
            }
        }
    }
}

impl Scene for BattleScene {
    fn handle_key(&mut self, keycode: Keycode) {
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
        };
    }

    fn handle_mouse(&mut self, x: i32, y: i32, button: Option<MouseButton>) {
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
            };
        }
    }

    fn render(&self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        self.ecs.write_resource::<FrameComponent>().current_frame = frame;

        canvas.set_draw_color(Color::from((0, 128, 255)));
        canvas.clear();

        for view in self.views.iter() {
            &view.render(&self.ecs, canvas, frame, &ContextData::None)?;
        }

        canvas.present();
        Ok(())
    }

    fn tick(&mut self, frame: u64) {
        process_tick_events(&mut self.ecs, frame);

        if !battle_actions::has_animations_blocking(&self.ecs) {
            tick_next_action(&mut self.ecs);
        }
    }

    fn on_quit(&mut self) -> BoxResult<()> {
        // Complete any outstanding animations to prevent any weirdness on load
        complete_animations(&mut self.ecs);

        Ok(())
    }

    fn get_state(&self) -> &World {
        &self.ecs
    }
}

pub fn process_tick_events(ecs: &mut World, frame: u64) {
    ecs.maintain();
    if ecs.try_fetch::<PlayerDeadComponent>().is_none() {
        tick_animations(ecs, frame);
        reap_killed(ecs);
    }
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
