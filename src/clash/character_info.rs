use serde::{Deserialize, Serialize};

use super::{Defenses, Temperature};

#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterInfo {
    pub defenses: Defenses,
    pub temperature: Temperature,
}

impl CharacterInfo {
    pub fn init(defenses: Defenses, temperature: Temperature) -> CharacterInfo {
        CharacterInfo { defenses, temperature }
    }
}
