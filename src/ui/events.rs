use bevy_ecs::prelude::*;
use ggez::mint::Vector2;

use crate::core::{AnimationState, SizedPoint};

pub struct SpriteAnimateActionEvent {
    pub entity: Entity,
    pub state: AnimationState,
}

impl SpriteAnimateActionEvent {
    pub fn new(entity: Entity, state: AnimationState) -> Self {
        SpriteAnimateActionEvent { entity, state }
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub struct SpriteAnimateActionCompleteEvent {
    pub entity: Entity,
}

impl SpriteAnimateActionCompleteEvent {
    pub fn new(entity: Entity) -> Self {
        SpriteAnimateActionCompleteEvent { entity }
    }
}

pub struct MovementAnimationEvent {
    pub entity: Entity,
    pub start: Vector2<f32>,
    pub end: Vector2<f32>,
}

impl MovementAnimationEvent {
    pub fn new(entity: Entity, start: Vector2<f32>, end: Vector2<f32>) -> Self {
        MovementAnimationEvent { entity, start, end }
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub struct MovementAnimationComplete {
    pub entity: Entity,
}

impl MovementAnimationComplete {
    pub fn new(entity: Entity) -> Self {
        MovementAnimationComplete { entity }
    }
}
