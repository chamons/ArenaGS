use bevy_ecs::prelude::*;
use ggez::{self, graphics::Canvas, input::keyboard::KeyInput};
use serde::{Deserialize, Serialize};

use super::{battle_scene, debug_overlay};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SceneKind {
    Battle,
    DebugOverlay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scenes {
    scenes: Vec<SceneKind>,
}

impl Scenes {
    pub fn new() -> Self {
        Scenes { scenes: vec![] }
    }

    pub fn push(&mut self, scene: SceneKind) {
        self.scenes.push(scene)
    }

    pub fn pop(&mut self) -> SceneKind {
        self.scenes.pop().unwrap()
    }

    #[allow(dead_code)]
    pub fn current(&self) -> SceneKind {
        *self.scenes.last().unwrap()
    }

    #[allow(dead_code)]
    pub fn all(&self) -> Vec<SceneKind> {
        self.scenes.clone()
    }

    // This kind of manual dispatch is ugly, but it allows hot swapping
    // and serialization of UI state

    pub fn update(state: SceneKind, world: &mut World, ctx: &mut ggez::Context) {
        match state {
            SceneKind::Battle => battle_scene::update(world, ctx),
            SceneKind::DebugOverlay => debug_overlay::update(world, ctx),
        }
    }

    pub fn draw(scenes: &mut [SceneKind], world: &mut World, ctx: &mut ggez::Context, canvas: &mut Canvas) {
        assert!(!scenes.is_empty());
        if let Some((current, rest)) = scenes.split_last_mut() {
            let draw_previous = match current {
                SceneKind::Battle => battle_scene::draw_previous(),
                SceneKind::DebugOverlay => debug_overlay::draw_previous(),
            };
            if draw_previous {
                Scenes::draw(rest, world, ctx, canvas);
            }
            match current {
                SceneKind::Battle => {
                    battle_scene::draw(world, ctx, canvas);
                }
                SceneKind::DebugOverlay => {
                    debug_overlay::draw(world, ctx, canvas);
                }
            }
        }
    }

    pub fn mouse_button_down_event(state: SceneKind, _world: &mut World, _ctx: &mut ggez::Context, _button: ggez::event::MouseButton, _x: f32, _y: f32) {
        match state {
            SceneKind::Battle => {}
            SceneKind::DebugOverlay => {}
        }
    }

    pub fn mouse_button_up_event(state: SceneKind, world: &mut World, ctx: &mut ggez::Context, button: ggez::event::MouseButton, x: f32, y: f32) {
        match state {
            SceneKind::Battle => {}
            SceneKind::DebugOverlay => debug_overlay::mouse_button_up_event(world, ctx, button, x, y),
        }
    }

    pub fn mouse_motion_event(state: SceneKind, _world: &mut World, _ctx: &mut ggez::Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        match state {
            SceneKind::Battle => {}
            SceneKind::DebugOverlay => {}
        }
    }

    pub fn mouse_enter_or_leave(state: SceneKind, _world: &mut World, _ctx: &mut ggez::Context, _entered: bool) {
        match state {
            SceneKind::Battle => {}
            SceneKind::DebugOverlay => {}
        }
    }

    pub fn mouse_wheel_event(state: SceneKind, _world: &mut World, _ctx: &mut ggez::Context, _x: f32, _y: f32) {
        match state {
            SceneKind::Battle => {}
            SceneKind::DebugOverlay => {}
        }
    }

    pub fn key_down_event(state: SceneKind, _world: &mut World, _ctx: &mut ggez::Context, _input: ggez::input::keyboard::KeyInput, _repeated: bool) {
        match state {
            SceneKind::Battle => {}
            SceneKind::DebugOverlay => {}
        }
    }

    pub fn key_up_event(state: SceneKind, world: &mut World, ctx: &mut ggez::Context, input: KeyInput) {
        match state {
            SceneKind::Battle => battle_scene::key_up_event(world, ctx, input),
            SceneKind::DebugOverlay => debug_overlay::key_up_event(world, ctx, input),
        }
    }
}
