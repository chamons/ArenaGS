use serde::{Deserialize, Serialize};

use super::Defenses;

#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterInfo {
    pub defenses: Defenses,
}

impl CharacterInfo {
    pub fn init(defenses: Defenses) -> CharacterInfo {
        CharacterInfo { defenses }
    }
}
