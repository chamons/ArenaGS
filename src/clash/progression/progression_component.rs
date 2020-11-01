use std::collections::HashSet;

use super::Equipment;
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
    pub fn init_gunslinger() -> ProgressionState {
        ProgressionState::init(0, 0, &[], CharacterWeaponKind::Gunslinger, Equipment::init(3, 2, 2, 1))
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
