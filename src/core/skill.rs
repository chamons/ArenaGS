use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SkillEffect {
    Move,
    RangedAttack,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum TargetType {
    None,
    Tile,
    Player,
    Enemy,
    Any,
    AnyoneButSelf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Skill {
    pub name: String,
    pub kind: SkillEffect,
    pub target: TargetType,
    pub range: Option<u32>,
    pub must_be_clear: bool,
}

impl Skill {
    pub fn new(name: &str, kind: SkillEffect, target: TargetType) -> Self {
        Skill {
            name: name.to_string(),
            kind,
            target,
            range: None,
            must_be_clear: false,
        }
    }

    pub fn with_range(mut self, range: u32) -> Skill {
        self.range = Some(range);
        self
    }

    pub fn must_be_clear(mut self) -> Skill {
        self.must_be_clear = true;
        self
    }
}
#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Skills {
    pub skills: Vec<Skill>,
}

impl Skills {
    pub fn new(skills: &[Skill]) -> Self {
        Skills { skills: Vec::from(skills) }
    }
}
