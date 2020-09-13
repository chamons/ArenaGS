// The ai macros can add "unnecessary" returns occationally
#![allow(clippy::needless_return)]

use std::collections::HashMap;

use specs::prelude::*;

use super::super::*;

pub fn elementalist_skills(_m: &mut HashMap<&'static str, SkillInfo>) {}
pub fn elementalist_action(ecs: &mut World, enemy: &Entity) {
    wait(ecs, *enemy);
}
pub fn water_elemental_action(ecs: &mut World, enemy: &Entity) {
    wait(ecs, *enemy);
}
pub fn fire_elemental_action(ecs: &mut World, enemy: &Entity) {
    wait(ecs, *enemy);
}

pub fn wind_elemental_action(ecs: &mut World, enemy: &Entity) {
    wait(ecs, *enemy);
}
pub fn earth_elemental_action(ecs: &mut World, enemy: &Entity) {
    wait(ecs, *enemy);
}
