use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

use self::points::SizedPoint;

mod frame_count;
mod points;

#[derive(Component, Debug, Deserialize, Serialize)]
struct Position {
    pub position: SizedPoint,
}

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
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
enum AppearanceKind {
    Gunslinger,
    Golem,
}

#[derive(Component, Debug, Deserialize, Serialize)]
struct Appearance {
    kind: AppearanceKind,
}

impl Appearance {
    pub fn new(kind: AppearanceKind) -> Self {
        Appearance { kind }
    }
}

pub fn create_game_world() -> World {
    let mut world = World::new();
    world.insert_resource(frame_count::Frame::zero());

    world.spawn().insert(Position::new(10, 7)).insert(Appearance::new(AppearanceKind::Gunslinger));
    world
        .spawn()
        .insert(Position::new_sized(2, 7, 2, 2))
        .insert(Appearance::new(AppearanceKind::Gunslinger));

    world
}

pub fn create_game_schedule() -> Schedule {
    let mut schedule = Schedule::default();

    let gameplay = SystemStage::single_threaded().with_system(frame_count::update_frame_count);
    schedule.add_stage("gameplay", gameplay);

    schedule
}
