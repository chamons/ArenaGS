use serde::{Deserialize, Serialize};

use super::{Defenses, Strength};

#[derive(Serialize, Deserialize, Clone)]
pub struct Character {
    pub defenses: Defenses,
}

impl Character {
    pub fn init(defenses: Defenses) -> Character {
        Character { defenses }
    }
}
