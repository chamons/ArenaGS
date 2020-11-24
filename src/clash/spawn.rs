use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::content::spawner;
use super::find_clear_landing;
use super::{DurationComponent, PlayerAlly, ShortInfo};
use crate::atlas::prelude::*;

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
    ShadowGunSlinger,
}

pub fn spawn(ecs: &mut World, target: SizedPoint, kind: SpawnKind, is_player_ally: bool, duration: Option<u32>) {
    let target = find_clear_landing(ecs, &target, None);
    let spawn = match kind {
        SpawnKind::Egg => spawner::bird_monster_add_egg(ecs, target),
        SpawnKind::BirdSpawn => spawner::bird_monster_add(ecs, target),
        SpawnKind::WaterElemental => spawner::water_elemental(ecs, target.origin, 0),
        SpawnKind::FireElemental => spawner::fire_elemental(ecs, target.origin, 0),
        SpawnKind::WindElemental => spawner::wind_elemental(ecs, target.origin, 0),
        SpawnKind::EarthElemental => spawner::earth_elemental(ecs, target.origin, 0),
        SpawnKind::ShadowGunSlinger => spawner::shadow_gunslinger(ecs, target.origin),
        _ => panic!("Can not spawn {:?} during combat", kind),
    };
    if is_player_ally {
        ecs.shovel(spawn, PlayerAlly::init());
    }
    if let Some(duration) = duration {
        ecs.shovel(spawn, DurationComponent::init(duration));
    }
}

pub fn spawn_replace(ecs: &mut World, invoker: Entity, kind: SpawnKind, is_player_ally: bool) {
    let position = ecs.get_position(invoker);
    ecs.delete_entity(invoker).unwrap();
    spawn(ecs, position, kind, is_player_ally, None);
}
