use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{self, PathBuf},
};

use anyhow::Result;
use bevy_ecs::prelude::*;
use ggez::{
    event::EventHandler,
    graphics::{self, Color},
    Context, GameResult,
};

use crate::core;

use super::{BattleScene, ImageCache, SceneStack};

pub struct GameState {
    world: World,
    schedule: Schedule,
    scenes: SceneStack<World>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Result<GameState> {
        let mut world = core::create_game_world(&mut ctx.fs)?;

        ctx.gfx
            .add_font("default", graphics::FontData::from_path(ctx, "/fonts/LibreFranklin-Regular.ttf")?);

        world.insert_resource(ImageCache::load(ctx, path::PathBuf::from("/"))?);

        let schedule = core::create_game_schedule();
        let mut scenes = SceneStack::new();
        scenes.push(Box::new(BattleScene::new()));

        Ok(GameState { world, schedule, scenes })
    }
}

const FPS: u32 = 60;

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ctx.time.check_update_time(FPS) {
            self.schedule.run_once(&mut self.world);
        }
        self.scenes.update(&mut self.world, ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        // Because pixel art
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.scenes.draw(&mut self.world, ctx, &mut canvas);

        canvas.finish(ctx)
    }
}
