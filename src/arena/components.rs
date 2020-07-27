use specs::prelude::*;
use specs_derive::Component;
use std::fmt;

mod render;
pub use render::{RenderComponent, RenderOrder, SpriteKinds};

use crate::atlas::Point;
use crate::clash::{FrameComponent, TargetType};

#[derive(Clone)]
pub enum BattleTargetSource {
    Skill(String),
}

#[derive(is_enum_variant, Clone, Debug)]
pub enum DebugKind {
    MapOverlay(),
}

impl fmt::Display for DebugKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(is_enum_variant, Clone)]
pub enum BattleSceneState {
    Default(),
    Targeting(BattleTargetSource, TargetType),
    Debug(DebugKind),
}

#[derive(Component)]
pub struct BattleSceneStateComponent {
    pub state: BattleSceneState,
}

impl BattleSceneStateComponent {
    pub fn init() -> BattleSceneStateComponent {
        BattleSceneStateComponent {
            state: BattleSceneState::Default(),
        }
    }
}
#[derive(Component)]
pub struct MousePositionComponent {
    pub position: Point,
}

impl MousePositionComponent {
    pub fn init() -> MousePositionComponent {
        MousePositionComponent { position: Point::init(0, 0) }
    }
}

pub trait UIState {
    fn get_current_frame(&self) -> u64;
    fn get_mouse_position(&self) -> Point;
}

impl UIState for World {
    fn get_current_frame(&self) -> u64 {
        self.read_resource::<FrameComponent>().current_frame
    }
    fn get_mouse_position(&self) -> Point {
        self.read_resource::<MousePositionComponent>().position
    }
}
