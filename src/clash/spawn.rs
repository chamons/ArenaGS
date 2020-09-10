use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::find_clear_landing;
use super::ShortInfo;
use crate::atlas::SizedPoint;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SpawnKind {
    Player,
    Bird,
    BirdSpawn,
    Egg,
}

pub fn spawn(ecs: &mut World, target: SizedPoint, kind: SpawnKind) {
    let target = find_clear_landing(ecs, &target, None);
    match kind {
        SpawnKind::Egg => super::content::spawner::bird_monster_add_egg(ecs, target),
        SpawnKind::BirdSpawn => super::content::spawner::bird_monster_add(ecs, target),
        _ => panic!("Can not spawn {:?} during combat", kind),
    }
}

pub fn spawn_replace(ecs: &mut World, invoker: &Entity, kind: SpawnKind) {
    let position = ecs.get_position(invoker);
    ecs.delete_entity(*invoker).unwrap();
    spawn(ecs, position, kind);
}
