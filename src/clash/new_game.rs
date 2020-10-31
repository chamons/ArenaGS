use std::cmp;
use std::path::Path;

use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use rand::Rng;
use specs::prelude::*;

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

pub fn create_random_battle(ecs: &mut World, progression_world: World) {
    let progression = progression_world.read_resource::<ProgressionComponent>().state.clone();
    let (kind, difficulty) = match progression.phase {
        0 => (BattleKind::SimpleGolem, 0),
        1 => (BattleKind::Bird, 0),
        2 => (BattleKind::Elementalist, 0),
        _ => {
            // Since we are creating an entire new world, it is acceptable to use thread RNG
            let mut random = rand::thread_rng();
            (random.gen(), progression.phase - 3)
        }
    };

    create_battle(ecs, progression, kind, difficulty);
}

fn create_battle(ecs: &mut World, progression: ProgressionState, kind: BattleKind, difficulty: u32) {
    let mut skills = SkillsResource::init();

    if progression.phase == 0 {
        ecs.log("Welcome to ArenaGS!");
        ecs.log("Press F1 for help.");
    }

    let map_data_path = Path::new(&get_exe_folder()).join("maps").join("beach").join("map1.dat");
    let map_data_path = map_data_path.stringify();
    ecs.insert(MapComponent::init(Map::init(map_data_path)));
    ecs.insert(ProgressionComponent::init(progression));
    ecs.insert(EquipmentResource::init_with(&content::gunslinger::get_equipment()));

    let player_position = find_placement(&ecs, 1, 1);
    progression::embattle::create_player(ecs, &mut skills, player_position);

    match kind {
        BattleKind::SimpleGolem => {
            let enemy_position = find_placement(&ecs, 1, 1);
            spawner::simple_golem(ecs, enemy_position);
            super::content::tutorial::golem_skills(&mut skills);
        }
        BattleKind::Bird => {
            let enemy_position = find_placement(&ecs, 2, 2);
            spawner::bird_monster(ecs, enemy_position, difficulty);
            super::content::bird::bird_skills(&mut skills);
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
                    ElementalKind::Water => spawner::water_elemental(ecs, enemy_position, difficulty),
                    ElementalKind::Fire => spawner::fire_elemental(ecs, enemy_position, difficulty),
                    ElementalKind::Wind => spawner::wind_elemental(ecs, enemy_position, difficulty),
                    ElementalKind::Earth => spawner::earth_elemental(ecs, enemy_position, difficulty),
                }
            }
            let enemy_position = find_placement(&ecs, 1, 1);
            spawner::elementalist(ecs, enemy_position, difficulty);
            super::content::elementalist::elementalist_skills(&mut skills);
        }
    }

    ecs.insert(skills);
}

pub fn new_game_intermission_state() -> World {
    let mut base_state = World::new();
    base_state.insert(ProgressionComponent::init(ProgressionState::init_empty()));

    create_intermission_state(&base_state)
}

pub fn create_intermission_state(battle_state: &World) -> World {
    let mut ecs = World::new();
    ecs.insert(crate::props::MousePositionComponent::init());
    ecs.insert(ProgressionComponent::init(battle_state.read_resource::<ProgressionComponent>().state.clone()));

    // TODO - Still wrong, should be calculated based on current equipped and reset when changed
    let mut m = SkillsResource::init();
    super::embattle::add_player_skills(&mut ecs, &mut m);
    ecs.insert(m);

    ecs
}
