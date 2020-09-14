// The ai macros can add "unnecessary" returns occationally
#![allow(clippy::needless_return)]

use std::collections::HashMap;

use specs::prelude::*;

use super::super::*;
use crate::{do_behavior, try_behavior};

pub fn golem_skills(m: &mut HashMap<&'static str, SkillInfo>) {
    
}

pub fn golem_action(ecs: &mut World, enemy: &Entity) {
    wait(ecs, *enemy);
}
