use std::fs::File;
use std::io::Read;
use std::io::Write;

use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::{BehaviorComponent, CharacterInfoComponent, FieldComponent, OrbComponent, PlayerComponent, PositionComponent};
use crate::atlas::{BoxResult, Point};

pub const MAX_MAP_TILES: u32 = crate::atlas::MAX_POINT_SIZE;
pub const MAX_MAP_TILES_SIZED: usize = MAX_MAP_TILES as usize;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct MapTile {
    walkable: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    tiles: [[MapTile; MAX_MAP_TILES_SIZED]; MAX_MAP_TILES_SIZED],
}

impl Map {
    pub fn init(filename: &str) -> BoxResult<Map> {
        let mut data = Vec::new();
        let mut file = File::open(filename)?;
        file.read_to_end(&mut data)?;
        let tiles = bincode::deserialize(&data)?;
        Ok(Map { tiles })
    }

    #[allow(dead_code)]
    pub const fn init_empty() -> Map {
        Map {
            tiles: [[MapTile { walkable: true }; MAX_MAP_TILES_SIZED]; MAX_MAP_TILES_SIZED],
        }
    }

    pub fn is_walkable(&self, position: &Point) -> bool {
        self.tiles[position.x as usize][position.y as usize].walkable
    }

    pub fn set_walkable(&mut self, position: &Point, walkable: bool) {
        self.tiles[position.x as usize][position.y as usize].walkable = walkable
    }

    pub fn write_to_file(&self) -> BoxResult<()> {
        let mut file = File::create("map.dat")?;
        let data = bincode::serialize(&self.tiles)?;
        file.write_all(&data)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum MapHitTestResult {
    None,
    Enemy,
    Orb,
    Field,
}

pub fn element_at_location(ecs: &World, map_position: &Point) -> MapHitTestResult {
    let positions = ecs.read_storage::<PositionComponent>();
    let orbs = ecs.read_storage::<OrbComponent>();
    let character_infos = ecs.read_storage::<CharacterInfoComponent>();
    let player = ecs.read_storage::<PlayerComponent>();
    let fields = ecs.read_storage::<FieldComponent>();
    let behaviors = ecs.read_storage::<BehaviorComponent>();

    for (position, character, player, orb, field, behavior) in (
        &positions,
        (&character_infos).maybe(),
        (&player).maybe(),
        (&orbs).maybe(),
        (&fields).maybe(),
        (&behaviors).maybe(),
    )
        .join()
    {
        if position.position.contains_point(map_position) {
            if let Some(_character) = character {
                if player.is_none() {
                    return MapHitTestResult::Enemy;
                }
            } else if orb.is_some() {
                return MapHitTestResult::Orb;
            } else if field.is_some() && behavior.is_some() {
                return MapHitTestResult::Field;
            }
        }
        if let Some(field) = field {
            if behavior.is_some()
                && field.fields.iter().any(|(p, _)| {
                    if let Some(p) = p {
                        p.x == map_position.x && p.y == map_position.y
                    } else {
                        false
                    }
                })
            {
                return MapHitTestResult::Field;
            }
        }
    }

    MapHitTestResult::None
}

#[cfg(test)]
mod tests {
    use super::super::{create_test_state, BehaviorKind, FieldComponent};
    use super::*;
    use crate::atlas::SizedPoint;

    #[test]
    fn map_hittest() {
        let mut ecs = create_test_state().with_player(2, 2, 0).with_character(3, 2, 0).build();
        ecs.create_entity()
            .with(PositionComponent::init(SizedPoint::init(4, 2)))
            .with(FieldComponent::init_single(255, 0, 0))
            .with(BehaviorComponent::init(BehaviorKind::None))
            .build();

        assert_eq!(MapHitTestResult::None, element_at_location(&ecs, &Point::init(1, 2)));
        assert_eq!(MapHitTestResult::Enemy, element_at_location(&ecs, &Point::init(3, 2)));
        assert_eq!(MapHitTestResult::Field, element_at_location(&ecs, &Point::init(4, 2)));
    }
}
