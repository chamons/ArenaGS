use serde::{Deserialize, Serialize};

use super::{Defenses, Temperature};

#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterInfo {
    pub name: String,
    pub defenses: Defenses,
    pub temperature: Temperature,
    pub skill_power: u32,
}

impl CharacterInfo {
    pub fn init(name: &str, defenses: Defenses, temperature: Temperature, skill_power: u32) -> CharacterInfo {
        CharacterInfo {
            name: name.to_string(),
            defenses,
            temperature,
            skill_power,
        }
    }
}
