use specs::prelude::*;
use specs_derive::Component;

use super::{CharacterInfoComponent, FieldComponent, PlayerComponent, Point, PositionComponent};

pub const MAX_MAP_TILES: u32 = 13;
pub const TOTAL_TILES: usize = (MAX_MAP_TILES * MAX_MAP_TILES) as usize;

#[derive(Copy, Clone)]
pub struct MapTile {
    walkable: bool,
}

pub struct Map {
    tiles: [MapTile; TOTAL_TILES],
}

impl Map {
    pub const fn init_empty() -> Map {
        Map {
            tiles: [MapTile { walkable: false }; TOTAL_TILES],
        }
    }

    pub fn is_walkable(&self, position: Point) -> bool {
        self.tiles[(position.x + (MAX_MAP_TILES * position.y)) as usize].walkable
    }
}

#[derive(Component)]
pub struct MapComponent {
    map: Map,
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
        if position.x == map_position.x as u32 && position.y == map_position.y as u32 {
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
