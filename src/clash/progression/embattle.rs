use specs::prelude::*;

use crate::atlas::prelude::*;
use crate::clash::content::{gunslinger, spawner};
use crate::clash::*;

pub fn create_player(ecs: &mut World, skills: &mut SkillsResource, player_position: Point) {
    spawner::player(ecs, player_position);
    gunslinger::gunslinger_skills(skills);
}
