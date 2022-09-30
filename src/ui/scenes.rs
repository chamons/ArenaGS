use bevy_ecs::prelude::*;
use ggez::{self, graphics::Canvas, input::keyboard::KeyInput};
use serde::{Deserialize, Serialize};

use crate::ui::targeting_draw_previous;

use super::targeting_key_up_event;
#[cfg(not(feature = "hotreload"))]
use super::{battle_scene::*, debug_overlay::*, target_overlay::*};

#[cfg(feature = "hotreload")]
use systems_hot::*;

#[cfg(feature = "hotreload")]
#[hot_lib_reloader::hot_module(dylib = "arenalib")]
mod systems_hot {
    use bevy_ecs::prelude::*;
    use ggez::graphics::Canvas;
    use ggez::input::keyboard::KeyInput;

    hot_functions_from_file!("src/ui/battle/battle_scene.rs");
    hot_functions_from_file!("src/ui/battle/debug_overlay.rs");
    hot_functions_from_file!("src/ui/battle/target_overlay.rs");
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SceneKind {
    Battle,
    Target,
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
    // All scene calls must be marked #[no_mangle] for hot reloading to work!
    pub fn update(state: SceneKind, world: &mut World, ctx: &mut ggez::Context) {
        match state {
            SceneKind::Battle => battle_update(world, ctx),
            SceneKind::DebugOverlay => debug_update(world, ctx),
            SceneKind::Target => targeting_update(world, ctx),
        }
    }

    pub fn draw(scenes: &mut [SceneKind], world: &mut World, ctx: &mut ggez::Context, canvas: &mut Canvas) {
        assert!(!scenes.is_empty());
        if let Some((current, rest)) = scenes.split_last_mut() {
            let draw_previous = match current {
                SceneKind::Battle => battle_draw_previous(),
                SceneKind::DebugOverlay => debug_draw_previous(),
                SceneKind::Target => targeting_draw_previous(),
            };
            if draw_previous {
                Scenes::draw(rest, world, ctx, canvas);
            }
            match current {
                SceneKind::Battle => battle_draw(world, ctx, canvas),
                SceneKind::DebugOverlay => debug_draw(world, ctx, canvas),
                SceneKind::Target => targeting_draw(world, ctx, canvas),
            }
        }
    }

    pub fn mouse_button_down_event(state: SceneKind, _world: &mut World, _ctx: &mut ggez::Context, _button: ggez::event::MouseButton, _x: f32, _y: f32) {
        match state {
            SceneKind::Battle => {}
            SceneKind::DebugOverlay => {}
            SceneKind::Target => {}
        }
    }

    pub fn mouse_button_up_event(state: SceneKind, world: &mut World, ctx: &mut ggez::Context, button: ggez::event::MouseButton, x: f32, y: f32) {
        match state {
            SceneKind::Battle => {}
            SceneKind::DebugOverlay => debug_mouse_button_up_event(world, ctx, button, x, y),
            SceneKind::Target => targeting_mouse_button_up_event(world, ctx, button, x, y),
        }
    }

    pub fn mouse_motion_event(state: SceneKind, _world: &mut World, _ctx: &mut ggez::Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        match state {
            SceneKind::Battle => {}
            SceneKind::DebugOverlay => {}
            SceneKind::Target => {}
        }
    }

    pub fn mouse_enter_or_leave(state: SceneKind, _world: &mut World, _ctx: &mut ggez::Context, _entered: bool) {
        match state {
            SceneKind::Battle => {}
            SceneKind::DebugOverlay => {}
            SceneKind::Target => {}
        }
    }

    pub fn mouse_wheel_event(state: SceneKind, _world: &mut World, _ctx: &mut ggez::Context, _x: f32, _y: f32) {
        match state {
            SceneKind::Battle => {}
            SceneKind::DebugOverlay => {}
            SceneKind::Target => {}
        }
    }

    pub fn key_down_event(state: SceneKind, _world: &mut World, _ctx: &mut ggez::Context, _input: ggez::input::keyboard::KeyInput, _repeated: bool) {
        match state {
            SceneKind::Battle => {}
            SceneKind::DebugOverlay => {}
            SceneKind::Target => {}
        }
    }

    pub fn key_up_event(state: SceneKind, world: &mut World, ctx: &mut ggez::Context, input: KeyInput) {
        match state {
            SceneKind::Battle => battle_key_up_event(world, ctx, input),
            SceneKind::DebugOverlay => debug_key_up_event(world, ctx, input),
            SceneKind::Target => targeting_key_up_event(world, ctx, input),
        }
    }
}
