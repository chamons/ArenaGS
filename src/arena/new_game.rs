use std::fs::{read_to_string, File};
#[cfg(test)]
use std::io::Read;
use std::io::Write;
use std::path::Path;

use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker, SimpleMarkerAllocator};
use specs_derive::Component;

use super::components::*;
use crate::atlas::{get_exe_folder, BoxResult, Direction, EasyPath, Point, SizedPoint, ToSerialize};
use crate::clash::*;

fn find_placement(ecs: &World, width: u32, height: u32) -> Point {
    let random = &mut ecs.fetch_mut::<RandomComponent>().rand;
    loop {
        let x = random.gen_range(2, 11);
        let y = random.gen_range(2, 11);
        let point = Point::init(x, y);
        if !is_area_clear(ecs, &SizedPoint::init_multi(x, y, width, height).all_positions(), None) {
            continue;
        }
        if find_all_characters(ecs).iter().any(|c| ecs.get_position(c).distance_to(point).unwrap_or(0) < 4) {
            continue;
        }

        let map = &ecs.read_resource::<MapComponent>().map;

        let directions = vec![Direction::North, Direction::West, Direction::South, Direction::East];
        if SizedPoint::init_multi(x, y, width, height)
            .all_positions()
            .iter()
            .any(|p| directions.iter().map(|d| d.point_in_direction(&p).unwrap()).any(|p| !map.is_walkable(&p)))
        {
            continue;
        }
        return point;
    }
}

pub enum BattleKind {
    Bird,
    Elementalist,
}

impl Distribution<BattleKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BattleKind {
        match rng.gen_range(0, 2) {
            0 => BattleKind::Bird,
            1 => BattleKind::Elementalist,
            _ => unreachable!(),
        }
    }
}

pub fn random_new_world(difficulty: u32) -> BoxResult<World> {
    // Since we are creating an entire new world, it is acceptable to use thread RNG
    let mut random = rand::thread_rng();
    let kind: BattleKind = random.gen();
    new_world(kind, difficulty)
}

pub fn new_world(kind: BattleKind, difficulty: u32) -> BoxResult<World> {
    let mut ecs = create_world();
    add_ui_extension(&mut ecs);

    let map_data_path = Path::new(&get_exe_folder()).join("maps").join("beach").join("map1.dat");
    let map_data_path = map_data_path.stringify();
    ecs.insert(MapComponent::init(Map::init(map_data_path)?));
    ecs.write_resource::<GameDifficultyComponent>().difficulty = difficulty;

    let player_position = find_placement(&ecs, 1, 1);
    crate::clash::content::spawner::player(&mut ecs, player_position);

    match kind {
        BattleKind::Bird => {
            let enemy_position = find_placement(&ecs, 2, 2);
            crate::clash::content::spawner::bird_monster(&mut ecs, enemy_position, difficulty);
        }
        BattleKind::Elementalist => {
            let enemy_position = find_placement(&ecs, 1, 1);
            crate::clash::content::spawner::elementalist(&mut ecs, enemy_position, difficulty);

            let enemy_position = find_placement(&ecs, 1, 1);
            crate::clash::content::spawner::water_elemental(&mut ecs, enemy_position, difficulty);

            let enemy_position = find_placement(&ecs, 1, 1);
            crate::clash::content::spawner::fire_elemental(&mut ecs, enemy_position, difficulty);

            let enemy_position = find_placement(&ecs, 1, 1);
            crate::clash::content::spawner::wind_elemental(&mut ecs, enemy_position, difficulty);

            let enemy_position = find_placement(&ecs, 1, 1);
            crate::clash::content::spawner::earth_elemental(&mut ecs, enemy_position, difficulty);
        }
    }

    map_background(&mut ecs);

    Ok(ecs)
}

pub fn map_background(ecs: &mut World) {
    ecs.create_entity()
        .with(RenderComponent::init(RenderInfo::init_with_order(
            SpriteKinds::BeachBackground,
            RenderOrder::Background,
        )))
        .marked::<SimpleMarker<ToSerialize>>()
        .build();
}
