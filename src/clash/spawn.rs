use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::content::spawner;
use super::find_clear_landing;
use super::ShortInfo;
use crate::atlas::SizedPoint;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SpawnKind {
    Player,
    Bird,
    BirdSpawn,
    Egg,
    WaterElemental,
    FireElemental,
    WindElemental,
    EarthElemental,
    Elementalist,
    SimpleGolem,
}

pub fn spawn(ecs: &mut World, target: SizedPoint, kind: SpawnKind) {
    let target = find_clear_landing(ecs, &target, None);
    match kind {
        SpawnKind::Egg => spawner::bird_monster_add_egg(ecs, target),
        SpawnKind::BirdSpawn => spawner::bird_monster_add(ecs, target),
        SpawnKind::WaterElemental => spawner::water_elemental(ecs, target.origin, 0),
        SpawnKind::FireElemental => spawner::fire_elemental(ecs, target.origin, 0),
        SpawnKind::WindElemental => spawner::wind_elemental(ecs, target.origin, 0),
        SpawnKind::EarthElemental => spawner::earth_elemental(ecs, target.origin, 0),
        _ => panic!("Can not spawn {:?} during combat", kind),
    }
}

pub fn spawn_replace(ecs: &mut World, invoker: &Entity, kind: SpawnKind) {
    let position = ecs.get_position(invoker);
    ecs.delete_entity(*invoker).unwrap();
    spawn(ecs, position, kind);
}
