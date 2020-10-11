use std::cmp;
use std::path::Path;

use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use rand::Rng;
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

use super::components::*;
use crate::atlas::get_exe_folder;
use crate::atlas::prelude::*;
use crate::clash::*;

use crate::clash::content::spawner;

fn find_placement(ecs: &World, width: u32, height: u32) -> Point {
    let random = &mut ecs.fetch_mut::<RandomComponent>().rand;
    loop {
        let x = random.gen_range(2, 11);
        let y = random.gen_range(2, 11);
        let point = Point::init(x, y);
        if !is_area_clear(ecs, &SizedPoint::init_multi(x, y, width, height).all_positions(), None) {
            continue;
        }
        if find_all_characters(ecs)
            .iter()
            .any(|&c| ecs.get_position(c).distance_to(point).unwrap_or(0) < 4)
        {
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
    SimpleGolem,
}

impl Distribution<BattleKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BattleKind {
        match rng.gen_range(0, 3) {
            0 => BattleKind::Bird,
            1 => BattleKind::Elementalist,
            2 => BattleKind::SimpleGolem,
            _ => unreachable!(),
        }
    }
}

pub fn random_new_world(phase: u32) -> World {
    let (kind, difficulty) = match phase {
        // 0 => (BattleKind::SimpleGolem, 0),
        // 1 => (BattleKind::Bird, 0),
        // 2 => (BattleKind::Elementalist, 0),
        _ => {
            // Since we are creating an entire new world, it is acceptable to use thread RNG
            let mut random = rand::thread_rng();
            (random.gen(), phase)
        }
    };

    create_battle(kind, difficulty, phase)
}

fn create_battle(kind: BattleKind, difficulty: u32, phase: u32) -> World {
    let mut ecs = create_world();
    add_ui_extension(&mut ecs);

    if phase == 0 {
        ecs.log("Welcome to ArenaGS!");
        ecs.log("Press F1 for help.");
    }

    let map_data_path = Path::new(&get_exe_folder()).join("maps").join("beach").join("map1.dat");
    let map_data_path = map_data_path.stringify();
    ecs.insert(MapComponent::init(Map::init(map_data_path)));
    ecs.write_resource::<GamePhaseComponent>().phase = phase;

    let player_position = find_placement(&ecs, 1, 1);
    spawner::player(&mut ecs, player_position);

    match kind {
        BattleKind::SimpleGolem => {
            let enemy_position = find_placement(&ecs, 1, 1);
            spawner::simple_golem(&mut ecs, enemy_position);
        }
        BattleKind::Bird => {
            let enemy_position = find_placement(&ecs, 2, 2);
            spawner::bird_monster(&mut ecs, enemy_position, difficulty);
        }
        BattleKind::Elementalist => {
            use crate::clash::content::elementalist::ElementalKind;
            // Since we are creating an entire new world, it is acceptable to use thread RNG
            let mut random = rand::thread_rng();
            let mut elements = vec![ElementalKind::Water, ElementalKind::Fire, ElementalKind::Wind, ElementalKind::Earth];
            for _ in 0..cmp::min(difficulty, 3) {
                elements.shuffle(&mut random);

                let enemy_position = find_placement(&ecs, 1, 1);
                match elements.pop().unwrap() {
                    ElementalKind::Water => spawner::water_elemental(&mut ecs, enemy_position, difficulty),
                    ElementalKind::Fire => spawner::fire_elemental(&mut ecs, enemy_position, difficulty),
                    ElementalKind::Wind => spawner::wind_elemental(&mut ecs, enemy_position, difficulty),
                    ElementalKind::Earth => spawner::earth_elemental(&mut ecs, enemy_position, difficulty),
                }
            }
            let enemy_position = find_placement(&ecs, 1, 1);
            spawner::elementalist(&mut ecs, enemy_position, difficulty);
        }
    }

    map_background(&mut ecs);

    ecs
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
