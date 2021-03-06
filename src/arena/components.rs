use std::fmt;

use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

mod render;
pub use render::{RenderInfo, RenderOrder, SpriteKinds};

use super::Animation;
use super::BattleActionRequest;
use crate::atlas::prelude::*;
use crate::clash::EventCoordinator;
use crate::props::MousePositionComponent;

#[derive(Clone, Serialize, Deserialize)]
pub enum BattleTargetSource {
    Skill(String),
}

#[derive(is_enum_variant, Clone, Debug, Serialize, Deserialize)]
pub enum DebugKind {
    MapOverlay(),
}

impl fmt::Display for DebugKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(is_enum_variant, Clone, Serialize, Deserialize)]
pub enum BattleSceneState {
    Default(),
    Targeting(BattleTargetSource),
    Debug(DebugKind),
}

#[derive(Component, ConvertSaveload, Clone)]
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

#[derive(Component)] // NotConvertSaveload
pub struct AnimationComponent {
    pub animation: Animation,
}

impl AnimationComponent {
    pub fn init(animation: Animation) -> AnimationComponent {
        AnimationComponent { animation }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct RenderComponent {
    pub render: RenderInfo,
}

impl RenderComponent {
    pub fn init(render: RenderInfo) -> RenderComponent {
        RenderComponent { render }
    }
}

#[derive(Component)] // NotConvertSaveload
pub struct BufferedInputComponent {
    pub input: Option<BattleActionRequest>,
}

impl BufferedInputComponent {
    pub fn init() -> BufferedInputComponent {
        BufferedInputComponent { input: None }
    }
}

#[derive(Component)] // NotConvertSaveload
pub struct AccelerateAnimationsComponent {
    pub state: bool,
}

impl AccelerateAnimationsComponent {
    pub fn init() -> AccelerateAnimationsComponent {
        AccelerateAnimationsComponent { state: false }
    }
}

pub enum LogIndexDelta {
    None,
    PageUp,
    PageDown,
    JumpToEnd,
}

#[derive(Component)] // NotConvertSaveload
pub struct LogIndexPosition {
    pub index: usize,
    pub delta: LogIndexDelta,
}

impl LogIndexPosition {
    pub fn init() -> LogIndexPosition {
        LogIndexPosition {
            index: 0,
            delta: LogIndexDelta::None,
        }
    }
}

pub fn add_ui_extension(ecs: &mut World) {
    ecs.register::<RenderComponent>();
    ecs.register::<BattleSceneStateComponent>();
    ecs.register::<MousePositionComponent>();
    ecs.register::<AnimationComponent>();
    ecs.register::<super::saveload::SerializationHelper>();
    // If you add additional components remember to update saveload.rs

    ecs.subscribe(super::battle_scene::create_view_event);
    ecs.subscribe(super::battle_animations::move_event);
    ecs.subscribe(super::battle_animations::battle_animation_event);
    ecs.subscribe(super::battle_animations::melee_cone_event);
    ecs.subscribe(super::battle_animations::field_event);
    ecs.subscribe(super::battle_animations::explode_event);
    ecs.subscribe(super::views::log_event);

    ecs.insert(BattleSceneStateComponent::init());
    ecs.insert(MousePositionComponent::init());
    ecs.insert(BufferedInputComponent::init());
    ecs.insert(AccelerateAnimationsComponent::init());
    ecs.insert(LogIndexPosition::init());
}

pub trait UIState {
    fn get_mouse_position(&self) -> Point;
}

impl UIState for World {
    fn get_mouse_position(&self) -> Point {
        self.read_resource::<MousePositionComponent>().position
    }
}
