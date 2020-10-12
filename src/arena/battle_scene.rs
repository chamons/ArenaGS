use std::rc::Rc;

use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::components::*;
use super::views::*;
use super::{battle_actions, force_complete_animations, tick_animations};
use crate::clash::*;

use super::{new_game, saveload};
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::conductor::{Scene, StageDirection};

pub struct BattleScene {
    ecs: World,
    views: Vec<Box<dyn View>>,
    help: HelpPopup,
}

impl BattleScene {
    pub fn init(render_context_holder: &RenderContextHolder, text_renderer: &Rc<TextRenderer>, phase: u32) -> BoxResult<BattleScene> {
        let ecs = new_game::random_new_world(phase);

        let render_context = &render_context_holder.borrow();
        let mut views: Vec<Box<dyn View>> = vec![
            Box::from(MapView::init(&render_context)?),
            Box::from(InfoBarView::init(SDLPoint::new(780, 20), &render_context, Rc::clone(&text_renderer))?),
            Box::from(SkillBarView::init(
                render_context,
                &ecs,
                SDLPoint::new(137, 25 + super::views::MAP_CORNER_Y as i32 + super::views::TILE_SIZE as i32 * 13i32),
                Rc::clone(&text_renderer),
            )?),
            Box::from(LogView::init(SDLPoint::new(780, 550), &render_context, Rc::clone(&text_renderer))?),
            Box::from(StatusBarView::init(&render_context, SDLPoint::new(24, 24))?),
        ];

        let help = HelpPopup::init(&render_context, Rc::clone(&text_renderer))?;

        if cfg!(debug_assertions) {
            views.push(Box::from(DebugView::init(SDLPoint::new(20, 20), Rc::clone(&text_renderer))?));
        }

        Ok(BattleScene { ecs, views, help })
    }

    fn handle_default_key(&mut self, keycode: Keycode) {
        if let Some(i) = is_keystroke_skill(keycode) {
            if let Some(name) = super::views::get_skill_name_on_skillbar(&self.ecs, i as usize) {
                battle_actions::request_action(&mut self.ecs, super::BattleActionRequest::SelectSkill(name))
            }
        }
        match keycode {
            Keycode::Up => battle_actions::request_action(&mut self.ecs, super::BattleActionRequest::Move(Direction::North)),
            Keycode::Down => battle_actions::request_action(&mut self.ecs, super::BattleActionRequest::Move(Direction::South)),
            Keycode::Left => battle_actions::request_action(&mut self.ecs, super::BattleActionRequest::Move(Direction::West)),
            Keycode::Right => battle_actions::request_action(&mut self.ecs, super::BattleActionRequest::Move(Direction::East)),
            Keycode::F1 => {
                self.help.enable(&self.ecs, None, HitTestResult::Text("Top Level Help".to_string()));
                self.help.force_size_large();
            }

            _ => {}
        }
        if cfg!(debug_assertions) {
            match keycode {
                Keycode::D => self.ecs.write_storage::<DefenseComponent>().grab_mut(find_player(&self.ecs)).defenses.health = 0,
                Keycode::K => {
                    for e in find_enemies(&self.ecs) {
                        self.ecs.write_storage::<DefenseComponent>().grab_mut(e).defenses.health = 0
                    }
                }
                Keycode::N => {
                    self.ecs = new_game::random_new_world(0);
                }
                Keycode::S => saveload::save_to_disk(&mut self.ecs),
                Keycode::L => {
                    if let Ok(new_world) = saveload::load_from_disk() {
                        self.ecs = new_world;
                    }
                }
                Keycode::F10 => {
                    battle_actions::set_action_state(&mut self.ecs, BattleSceneState::Debug(DebugKind::MapOverlay()));
                }
                _ => {}
            }
        }
    }

    fn handle_target_key(&mut self, keycode: Keycode) {
        if keycode == Keycode::Escape {
            battle_actions::reset_action_state(&mut self.ecs)
        }

        // If they select a skill, start a new target session just like
        if let Some(i) = is_keystroke_skill(keycode) {
            if let Some(name) = super::views::get_skill_name_on_skillbar(&self.ecs, i as usize) {
                battle_actions::request_action(&mut self.ecs, super::BattleActionRequest::SelectSkill(name));
            }
        }
    }

    fn handle_debug_key(&mut self, kind: DebugKind, keycode: Keycode) {
        if keycode == Keycode::Escape {
            battle_actions::reset_action_state(&mut self.ecs);
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
        let hit = self.views.iter().rev().filter_map(|v| v.hit_test(&self.ecs, x, y)).next();
        if button == MouseButton::Left {
            if let Some(HitTestResult::Skill(name)) = &hit {
                battle_actions::request_action(&mut self.ecs, super::BattleActionRequest::SelectSkill(name.to_string()))
            }
        }
        if button == MouseButton::Right {
            match &hit {
                Some(HitTestResult::Tile(target_position)) | Some(HitTestResult::Field(target_position)) => {
                    let player_position = self.ecs.get_position(find_player(&self.ecs));
                    if player_position.distance_to(*target_position).unwrap_or(0) == 1 {
                        battle_actions::request_action(
                            &mut self.ecs,
                            super::BattleActionRequest::Move(Direction::from_two_points(&player_position.origin, target_position)),
                        );
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_target_mouse(&mut self, x: i32, y: i32, button: MouseButton) {
        if button == MouseButton::Right {
            battle_actions::reset_action_state(&mut self.ecs);
            return;
        }

        let target_info = match battle_actions::read_action_state(&self.ecs) {
            BattleSceneState::Targeting(target_source) => Some(target_source),
            _ => None,
        };

        if let Some(target_source) = target_info {
            if button == MouseButton::Left {
                if let Some(map_position) = screen_to_map_position(x, y) {
                    match target_source {
                        BattleTargetSource::Skill(skill_name) => {
                            battle_actions::request_action(&mut self.ecs, super::BattleActionRequest::TargetSkill(skill_name, map_position))
                        }
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
    fn handle_key(&mut self, keycode: Keycode, _keymod: Mod) {
        if self.help.is_enabled() && keycode == Keycode::Escape {
            self.help.disable();
        }

        match keycode {
            Keycode::PageUp => {
                self.ecs.raise_event(EventKind::LogScrolled(LogDirection::Backwards), None);
            }
            Keycode::PageDown => {
                self.ecs.raise_event(EventKind::LogScrolled(LogDirection::Forward), None);
            }

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
        self.help.handle_mouse(&self.ecs, x, y, button);
        // Prevent stray clicks from passing through
        if self.help.is_enabled() {
            return;
        }

        if let Some(button) = button {
            if button == MouseButton::Middle {
                let hit = self.views.iter().rev().filter_map(|v| v.hit_test(&self.ecs, x, y)).next();
                if let Some(hit) = hit {
                    self.help.enable(&self.ecs, Some(Point::init(x as u32, y as u32)), hit);
                }
            }

            let state = battle_actions::read_action_state(&self.ecs);
            match state {
                BattleSceneState::Default() => self.handle_default_mouse(x, y, button),
                BattleSceneState::Targeting(_) => self.handle_target_mouse(x, y, button),
                BattleSceneState::Debug(kind) => self.handle_debug_mouse(kind, x, y, button),
            };
        }
    }

    fn render(&mut self, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        self.ecs.write_resource::<FrameComponent>().current_frame = frame;

        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();

        for view in self.views.iter() {
            view.render(&self.ecs, canvas, frame)?;
        }

        self.help.render(&self.ecs, canvas, frame)?;

        canvas.present();
        Ok(())
    }

    fn tick(&mut self, frame: u64) {
        battle_tick(&mut self.ecs, frame);
    }

    fn on_quit(&mut self) -> BoxResult<()> {
        // Complete any outstanding animations to prevent any weirdness on load
        force_complete_animations(&mut self.ecs);

        Ok(())
    }

    fn ask_stage_direction(&self) -> StageDirection {
        battle_stage_direction(&self.ecs)
    }
}

pub fn battle_tick(ecs: &mut World, frame: u64) {
    process_tick_events(ecs, frame);

    if !battle_actions::has_animations_blocking(ecs) {
        let player_can_act = tick_next_action(ecs);
        if player_can_act {
            battle_actions::process_any_queued_action(ecs);
            #[cfg(feature = "self_play")]
            {
                super::self_play::take_player_action(ecs);
            }
        }
    }
}

pub fn battle_stage_direction(ecs: &World) -> StageDirection {
    if ecs.try_fetch::<PlayerDeadComponent>().is_some() {
        return StageDirection::BattlePlayerDeath("This is where detailed death info goes".to_string());
    }
    let entities = ecs.read_resource::<specs::world::EntitiesRes>();
    let character_infos = ecs.read_storage::<CharacterInfoComponent>();
    let player = ecs.read_storage::<PlayerComponent>();

    let non_player_character_count = (&entities, &character_infos, (&player).maybe()).join().filter(|(_, _, p)| p.is_none()).count();
    if non_player_character_count == 0 {
        return StageDirection::BattleEnemyDefeated(ecs.read_resource::<GamePhaseComponent>().phase + 1);
    }
    StageDirection::Continue
}

pub fn process_tick_events(ecs: &mut World, frame: u64) {
    ecs.maintain();
    if ecs.try_fetch::<PlayerDeadComponent>().is_none() {
        tick_animations(ecs, frame);
        reap_killed(ecs);
    }
}

fn is_keystroke_skill(keycode: Keycode) -> Option<usize> {
    let name = keycode.name();
    let chars: Vec<char> = name.chars().collect();

    if chars.len() == 1 {
        match chars[0] {
            '0'..='9' => Some(hotkey_to_skill_index(chars[0].to_string().parse().unwrap())),
            _ => None,
        }
    } else {
        None
    }
}

pub fn create_view_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    use crate::after_image::CharacterAnimationState;

    match kind {
        EventKind::Creation(kind) => match kind {
            SpawnKind::Bird => ecs.shovel(target.unwrap(), RenderComponent::init(RenderInfo::init(SpriteKinds::MonsterBirdBrown))),
            SpawnKind::Egg => ecs.shovel(target.unwrap(), RenderComponent::init(RenderInfo::init(SpriteKinds::Egg))),
            SpawnKind::BirdSpawn => ecs.shovel(target.unwrap(), RenderComponent::init(RenderInfo::init(SpriteKinds::SmallMonsterBirdBrown))),
            SpawnKind::Elementalist => ecs.shovel(target.unwrap(), RenderComponent::init(RenderInfo::init(SpriteKinds::Elementalist))),
            SpawnKind::WaterElemental => ecs.shovel(target.unwrap(), RenderComponent::init(RenderInfo::init(SpriteKinds::WaterElemental))),
            SpawnKind::FireElemental => ecs.shovel(target.unwrap(), RenderComponent::init(RenderInfo::init(SpriteKinds::FireElemental))),
            SpawnKind::WindElemental => ecs.shovel(target.unwrap(), RenderComponent::init(RenderInfo::init(SpriteKinds::WindElemental))),
            SpawnKind::EarthElemental => ecs.shovel(target.unwrap(), RenderComponent::init(RenderInfo::init(SpriteKinds::EarthElemental))),
            SpawnKind::SimpleGolem => ecs.shovel(target.unwrap(), RenderComponent::init(RenderInfo::init(SpriteKinds::SimpleGolem))),
            SpawnKind::Player => ecs.shovel(
                target.unwrap(),
                RenderComponent::init(RenderInfo::init_with_char_state(
                    SpriteKinds::MaleBrownHairBlueBody,
                    CharacterAnimationState::Idle,
                )),
            ),
        },
        _ => {}
    }
}
