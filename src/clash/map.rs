use std::fs::File;
use std::io::Read;
use std::io::Write;

use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::Component;

use super::{CharacterInfoComponent, FieldComponent, PlayerComponent, Point, PositionComponent};
use crate::atlas::BoxResult;

pub const MAX_MAP_TILES: u32 = 13;
pub const MAX_MAP_TILES_SIZED: usize = MAX_MAP_TILES as usize;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct MapTile {
    walkable: bool,
}

#[derive(Serialize, Deserialize)]
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

    pub fn is_in_bounds(&self, position: &Point) -> bool {
        position.x < MAX_MAP_TILES && position.y < MAX_MAP_TILES
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

#[derive(Component)]
pub struct MapComponent {
    pub map: Map,
}

impl MapComponent {
    pub const fn init(map: Map) -> MapComponent {
        MapComponent { map }
    }
}

pub enum MapHitTestResult {
    None(),
    Enemy(),
    Player(),
    Field(),
}

pub fn element_at_location(ecs: &World, map_position: &Point) -> MapHitTestResult {
    let positions = ecs.read_storage::<PositionComponent>();
    let fields = ecs.read_storage::<FieldComponent>();
    let character_infos = ecs.read_storage::<CharacterInfoComponent>();
    let player = ecs.read_storage::<PlayerComponent>();

    for (position, field, character, player) in (&positions, (&fields).maybe(), (&character_infos).maybe(), (&player).maybe()).join() {
        if position.position == *map_position {
            if let Some(_character) = character {
                if player.is_none() {
                    return MapHitTestResult::Enemy();
                } else {
                    return MapHitTestResult::Player();
                }
            } else if let Some(_field) = field {
                return MapHitTestResult::Field();
            }
        }
    }
    MapHitTestResult::None()
}
