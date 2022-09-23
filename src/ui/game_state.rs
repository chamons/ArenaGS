use std::path;

use anyhow::Result;
use bevy_ecs::prelude::*;
use ggez::{
    event::EventHandler,
    graphics::{self, Color},
    Context, GameError, GameResult,
};

use crate::core;

use super::{BattleScene, ImageCache, SceneStack, ScreenScale};

pub struct GameState {
    world: World,
    schedule: Schedule,
    scenes: SceneStack<World>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Result<GameState> {
        let mut world = core::create_game_world(&mut ctx.fs)?;
        world.insert_resource(ScreenScale::new(ctx));

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
        canvas.set_screen_coordinates(graphics::Rect::new(0.0, 0.0, 1280.0, 960.0));

        self.scenes.draw(&mut self.world, ctx, &mut canvas);

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: ggez::event::MouseButton, x: f32, y: f32) -> Result<(), GameError> {
        self.scenes.mouse_button_down_event(&mut self.world, ctx, button, x, y);
        Ok(())
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: ggez::event::MouseButton, x: f32, y: f32) -> Result<(), GameError> {
        self.scenes.mouse_button_up_event(&mut self.world, ctx, button, x, y);
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) -> Result<(), GameError> {
        self.scenes.mouse_motion_event(&mut self.world, ctx, x, y, dx, dy);
        Ok(())
    }

    fn mouse_enter_or_leave(&mut self, ctx: &mut Context, entered: bool) -> Result<(), GameError> {
        self.scenes.mouse_enter_or_leave(&mut self.world, ctx, entered);
        Ok(())
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) -> Result<(), GameError> {
        self.scenes.mouse_wheel_event(&mut self.world, ctx, x, y);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: ggez::input::keyboard::KeyInput, repeated: bool) -> Result<(), GameError> {
        self.scenes.key_down_event(&mut self.world, ctx, input, repeated);
        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, input: ggez::input::keyboard::KeyInput) -> Result<(), GameError> {
        self.scenes.key_up_event(&mut self.world, ctx, input);
        Ok(())
    }
}
