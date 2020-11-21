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
    pub influence: u32,
    pub items: HashSet<String>,
    pub equipment_expansions: HashSet<String>,
    pub weapon: CharacterWeaponKind,
    pub equipment: Equipment,
}

impl ProgressionState {
    pub fn init_gunslinger() -> ProgressionState {
        ProgressionState::init(0, 0, &["Oversized Chamber"], CharacterWeaponKind::Gunslinger, Equipment::init(3, 2, 2, 1))
    }

    pub fn init(phase: u32, influence: u32, items: &[&str], weapon: CharacterWeaponKind, equipment: Equipment) -> ProgressionState {
        ProgressionState {
            phase,
            influence,
            items: items.iter().map(|s| s.to_string()).collect(),
            equipment_expansions: HashSet::new(),
            weapon,
            equipment,
        }
    }

    pub fn has_unlock(&self, name: &str) -> bool {
        self.items.contains(name) || self.equipment_expansions.contains(name)
    }
}
