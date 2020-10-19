use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Deserialize, Serialize, Clone)]
pub struct ProgressionComponent {
    pub state: ProgressionState,
}

impl ProgressionComponent {
    pub fn init(state: ProgressionState) -> ProgressionComponent {
        ProgressionComponent { state }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ProgressionState {
    pub phase: u32,
    pub skills: Vec<String>,
}

impl ProgressionState {
    pub fn init(phase: u32) -> ProgressionState {
        ProgressionState { phase, skills: vec![] }
    }
}

pub fn wrap_progression(state: &ProgressionState) -> World {
    let mut world = World::new();
    world.insert(ProgressionComponent::init(state.clone()));
    return world;
}
