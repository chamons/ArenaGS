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

mod field;
pub use field::*;

mod physics;
pub use physics::*;

mod ecs;
pub use ecs::*;

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Position {
    pub position: SizedPoint,
}

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Player;

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Character;

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

impl From<SizedPoint> for Position {
    fn from(position: SizedPoint) -> Self {
        Position { position }
    }
}

pub fn create_game_world(fs: &mut ggez::filesystem::Filesystem) -> Result<World> {
    let mut world = World::new();

    setup_game_resources(&mut world, fs)?;

    world
        .spawn()
        .insert(Character)
        .insert(crate::ui::Animation::new())
        .insert(Appearance::new(AppearanceKind::MaleBrownHairBlueBody))
        .insert(Position::new(8, 6))
        .insert(Player)
        .insert(Skills::new(&[
            Skill::new("Shoot", SkillEffect::RangedAttack, TargetType::Enemy)
                .with_range(24)
                .path_must_be_clear(),
            Skill::new("Dodge", SkillEffect::Move, TargetType::Tile).with_range(2).path_must_be_clear(),
        ]));

    world
        .spawn()
        .insert(Character)
        .insert(Appearance::new(AppearanceKind::MaleBrownHairBlueBody))
        .insert(crate::ui::Animation::new())
        .insert(Position::new(6, 6));

    world
        .spawn()
        .insert(Character)
        .insert(Position::new_sized(3, 4, 2, 2))
        .insert(crate::ui::Animation::new())
        .insert(Appearance::new(AppearanceKind::Golem));

    world
        .spawn()
        .insert(Fields::new(FieldColor::Gray, &[Point::new(7, 5), Point::new(7, 6), Point::new(7, 7)]));

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
