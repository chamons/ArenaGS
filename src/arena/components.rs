use std::fmt;

use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

mod render;
pub use render::{RenderInfo, RenderOrder, SpriteKinds};

use super::Animation;
use crate::atlas::Point;
use crate::clash::EventCoordinator;

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

#[derive(Component, ConvertSaveload, Clone)]
pub struct MousePositionComponent {
    pub position: Point,
}

impl MousePositionComponent {
    pub fn init() -> MousePositionComponent {
        MousePositionComponent { position: Point::init(0, 0) }
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

pub fn add_ui_extension(ecs: &mut World) {
    ecs.register::<RenderComponent>();
    ecs.register::<BattleSceneStateComponent>();
    ecs.register::<MousePositionComponent>();
    ecs.register::<AnimationComponent>();
    ecs.register::<super::saveload::SerializationHelper>();
    // If you add additional components remember to update saveload.rs

    ecs.subscribe(super::battle_animations::move_event);
    ecs.subscribe(super::battle_animations::bolt_event);
    ecs.subscribe(super::battle_animations::melee_event);
    ecs.subscribe(super::battle_animations::field_event);
    ecs.subscribe(super::battle_animations::explode_event);

    ecs.insert(BattleSceneStateComponent::init());
    ecs.insert(MousePositionComponent::init());
}

pub trait UIState {
    fn get_mouse_position(&self) -> Point;
}

impl UIState for World {
    fn get_mouse_position(&self) -> Point {
        self.read_resource::<MousePositionComponent>().position
    }
}
