use std::collections::HashSet;

use super::Equipment;
use crate::props::MousePositionComponent;
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
pub enum CharacterWeaponKind {
    Gunslinger,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ProgressionState {
    pub phase: u32,
    pub experience: u32,
    pub skills: HashSet<String>,
    pub weapon: CharacterWeaponKind,
    pub equipment: Equipment,
}

impl ProgressionState {
    pub fn init_empty() -> ProgressionState {
        ProgressionState::init(0, 0, &[], CharacterWeaponKind::Gunslinger, Equipment::init_empty())
    }

    pub fn init(phase: u32, experience: u32, skills: &[&str], weapon: CharacterWeaponKind, equipment: Equipment) -> ProgressionState {
        ProgressionState {
            phase,
            experience,
            skills: skills.iter().map(|s| s.to_string()).collect(),
            weapon,
            equipment,
        }
    }
}

pub fn wrap_progression(state: &ProgressionState) -> World {
    let mut world = World::new();
    // Just to make UI work easier
    world.insert(MousePositionComponent::init());
    world.insert(ProgressionComponent::init(state.clone()));

    {
        let v = world.read_resource::<ProgressionComponent>();
        let m = world.read_resource::<MousePositionComponent>();
    }

    world
}
