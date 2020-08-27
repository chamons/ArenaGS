use serde::{Deserialize, Serialize};

use super::{Defenses, Temperature};

#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterInfo {
    pub name: String,
    pub defenses: Defenses,
    pub temperature: Temperature,
}

impl CharacterInfo {
    pub fn init(name: &str, defenses: Defenses, temperature: Temperature) -> CharacterInfo {
        CharacterInfo {
            name: name.to_string(),
            defenses,
            temperature,
        }
    }
}
