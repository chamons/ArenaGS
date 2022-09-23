use bevy_ecs::prelude::*;
use keyframe::AnimationSequence;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum AppearanceKind {
    MaleBrownHairBlueBody,
    Golem,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum AnimationState {
    AttackOne,
    AttackTwo,
    Bow,
    Cheer,
    Crouch,
    Hit,
    Idle,
    Item,
    Magic,
    Status,
    Walk,
}

#[derive(Component, Deserialize, Serialize)]
pub struct Appearance {
    pub kind: AppearanceKind,
    pub state: AnimationState,

    #[serde(skip_serializing, skip_deserializing)]
    pub animation: Option<AnimationSequence<f32>>,
}

impl std::fmt::Debug for Appearance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Appearance").field("kind", &self.kind).field("state", &self.state).finish()
    }
}

impl Appearance {
    pub fn new(kind: AppearanceKind) -> Self {
        Appearance {
            kind,
            state: AnimationState::Idle,
            animation: None,
        }
    }
}
