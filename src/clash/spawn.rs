use serde::{Deserialize, Serialize};
use specs::prelude::*;

use crate::atlas::Point;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SpawnKind {
    Player,
    Bird,
    BirdSpawn,
}

pub fn begin_spawn(ecs: &mut World, _source: &Entity, target_position: Point, kind: SpawnKind) {
    match kind {
        SpawnKind::Bird => super::content::spawner::bird_monster_add(ecs, target_position),
        _ => panic!("Can not spawn {:?} during combat", kind),
    }
}
