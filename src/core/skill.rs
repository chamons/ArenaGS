use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SkillKind {
    Shoot,
    Dodge,
}

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Skill {
    pub kind: SkillKind,
}

impl Skill {
    pub fn new(kind: SkillKind) -> Self {
        Skill { kind }
    }
}
