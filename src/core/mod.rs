use anyhow::Result;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

mod map;
pub use map::*;

mod utils;
pub use utils::*;

mod appearance;
pub use appearance::*;

mod schedule;
pub use schedule::*;

mod log;
pub use log::*;

mod skill;
pub use skill::*;

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Position {
    pub position: SizedPoint,
}

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Player;

impl Position {
    pub const fn new(x: u32, y: u32) -> Self {
        Position {
            position: SizedPoint::new(x, y),
        }
    }

    pub const fn new_sized(x: u32, y: u32, width: u32, height: u32) -> Self {
        Position {
            position: SizedPoint::new_sized(x, y, width, height),
        }
    }

    pub fn origin(&self) -> Point {
        self.position.origin
    }
}

pub fn create_game_world(fs: &mut ggez::filesystem::Filesystem) -> Result<World> {
    let mut world = World::new();

    setup_game_resources(&mut world, fs)?;

    world
        .spawn()
        .insert(Position::new(6, 6))
        .insert(Appearance::new(AppearanceKind::MaleBrownHairBlueBody))
        .insert(Player);
    world
        .spawn()
        .insert(Position::new(8, 6))
        .insert(Appearance::new(AppearanceKind::MaleBrownHairBlueBody));

    world
        .spawn()
        .insert(Position::new_sized(3, 3, 2, 2))
        .insert(Appearance::new(AppearanceKind::Golem));

    Ok(world)
}

pub fn create_game_schedule() -> Schedule {
    let mut schedule = Schedule::default();

    schedule.add_stage("gameplay", gameplay_schedule());

    schedule
}

pub fn setup_game_resources(world: &mut World, fs: &mut ggez::filesystem::Filesystem) -> Result<()> {
    world.insert_resource(utils::Frame::zero());
    world.insert_resource(Log::new());

    let map = Map::load(&mut fs.open("/maps/beach/map1.dat")?)?;
    world.insert_resource(map);

    world.insert_resource(Events::<NewMessageEvent>::default());
    world.insert_resource(Events::<ScrollMessageEvent>::default());

    Ok(())
}

// Since we aren't using Bevy's App model, we have to clear our event buffers by hand
pub fn clear_event_buffers(mut a: ResMut<Events<NewMessageEvent>>, mut b: ResMut<Events<ScrollMessageEvent>>) {
    a.update();
    b.update();
}
