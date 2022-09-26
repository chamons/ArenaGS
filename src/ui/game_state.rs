use std::path;

use anyhow::Result;
use bevy_ecs::prelude::*;
use ggez::{
    event::EventHandler,
    graphics::{self, Color},
    Context, GameError, GameResult,
};

use crate::core;

use super::{ImageCache, SceneKind, Scenes, ScreenCoordinates};

pub struct GameState {
    world: World,
    schedule: Schedule,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Result<GameState> {
        let mut world = core::create_game_world(&mut ctx.fs)?;

        ctx.gfx
            .add_font("default", graphics::FontData::from_path(ctx, "/fonts/LibreFranklin-Regular.ttf")?);

        world.insert_resource(ScreenCoordinates::calculate(ctx));
        world.insert_resource(ImageCache::load(ctx, path::PathBuf::from("/"))?);
        super::setup_ui_resources(&mut world);

        let mut schedule = core::create_game_schedule();
        schedule.add_stage("ui", super::create_ui_schedule());

        let mut scenes = Scenes::new();
        scenes.push(SceneKind::Battle);
        world.insert_resource(scenes);

        Ok(GameState { world, schedule })
    }

    pub fn current_scene(&self) -> SceneKind {
        self.world.get_resource::<Scenes>().unwrap().current()
    }
}

const FPS: u32 = 60;

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ctx.time.check_update_time(FPS) {
            self.schedule.run_once(&mut self.world);
        }
        Scenes::update(self.current_scene(), &mut self.world, ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        // Because pixel art
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.world.get_resource::<ScreenCoordinates>().unwrap().set_screen(&mut canvas);

        Scenes::draw(&mut self.world.get_resource::<Scenes>().unwrap().all(), &mut self.world, ctx, &mut canvas);

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: ggez::event::MouseButton, x: f32, y: f32) -> Result<(), GameError> {
        Scenes::mouse_button_down_event(self.current_scene(), &mut self.world, ctx, button, x, y);
        Ok(())
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: ggez::event::MouseButton, x: f32, y: f32) -> Result<(), GameError> {
        Scenes::mouse_button_up_event(self.current_scene(), &mut self.world, ctx, button, x, y);
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) -> Result<(), GameError> {
        Scenes::mouse_motion_event(self.current_scene(), &mut self.world, ctx, x, y, dx, dy);
        Ok(())
    }

    fn mouse_enter_or_leave(&mut self, ctx: &mut Context, entered: bool) -> Result<(), GameError> {
        Scenes::mouse_enter_or_leave(self.current_scene(), &mut self.world, ctx, entered);
        Ok(())
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) -> Result<(), GameError> {
        Scenes::mouse_wheel_event(self.current_scene(), &mut self.world, ctx, x, y);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: ggez::input::keyboard::KeyInput, repeated: bool) -> Result<(), GameError> {
        Scenes::key_down_event(self.current_scene(), &mut self.world, ctx, input, repeated);
        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, input: ggez::input::keyboard::KeyInput) -> Result<(), GameError> {
        Scenes::key_up_event(self.current_scene(), &mut self.world, ctx, input);
        Ok(())
    }
}
