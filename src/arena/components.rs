use specs::prelude::*;
use specs_derive::Component;
use std::fmt;

mod animation;
pub use animation::{Animation, AnimationComponent};
mod render;
pub use render::{RenderComponent, RenderOrder, SpriteKinds};

use crate::clash::TargetType;

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
