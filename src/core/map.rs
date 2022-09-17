use std::io::{Read, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::points::Point;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum MapKind {
    Ashlands,
    Beach,
    Desert,
    Ruins,
    Winter,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct MapTile {
    walkable: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    tiles: [[MapTile; Map::MAX_TILES]; Map::MAX_TILES],
    kind: MapKind,
}

impl Map {
    pub const MAX_TILES: usize = super::points::MAX_POINT_SIZE as usize;

    pub fn load(file: &mut ggez::filesystem::File) -> Result<Map> {
        let mut data = vec![];
        file.read_to_end(&mut data)?;
        Ok(bincode::deserialize(&data)?)
    }

    #[allow(dead_code)]
    pub const fn empty(kind: MapKind) -> Map {
        Map {
            tiles: [[MapTile { walkable: true }; Map::MAX_TILES]; Map::MAX_TILES],
            kind,
        }
    }

    pub fn is_walkable(&self, position: &Point) -> bool {
        self.tiles[position.x as usize][position.y as usize].walkable
    }

    pub fn set_walkable(&mut self, position: &Point, walkable: bool) {
        self.tiles[position.x as usize][position.y as usize].walkable = walkable
    }

    #[allow(dead_code)]
    pub fn write_to_file(&self) -> Result<()> {
        let mut file = std::fs::File::create("map.dat")?;
        let data = bincode::serialize(&self)?;
        file.write_all(&data)?;
        Ok(())
    }
}
