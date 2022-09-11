use std::{env, path};

use anyhow::Result;
use ggez::event::{self, EventHandler, EventLoop};
use ggez::graphics::{self, Color};
use ggez::{conf, Context, ContextBuilder, GameResult};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoopWindowTarget;

struct MyGame {}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {}
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        canvas.finish(ctx)
    }
}

fn main() -> Result<()> {
    let mut cb = ContextBuilder::new("ArenaGS", "Chris Hamons")
        .window_setup(conf::WindowSetup {
            title: "Arena: Gunpowder and Sorcery".to_string(),
            ..Default::default()
        })
        .window_mode(conf::WindowMode {
            logical_size: Some(LogicalSize::new(1024.0, 768.0)),
            ..Default::default()
        });

    // Add ArenaGS-Data to the resource path
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("..");
        path.push("ArenaGS-Data");
        cb = cb.add_resource_path(path);
    }

    let (mut ctx, event_loop) = cb.build()?;
    let my_game = MyGame::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}
