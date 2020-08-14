use serde::{Deserialize, Serialize};

use super::{Defenses, Strength};

#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterInfo {
    pub defenses: Defenses,
}

impl CharacterInfo {
    pub fn init(defenses: Defenses) -> CharacterInfo {
        CharacterInfo { defenses }
    }
}
