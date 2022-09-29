#![allow(clippy::single_match)]
#![allow(clippy::collapsible_else_if)]

use std::{env, path};

use anyhow::Result;
use ggez::{conf, event, ContextBuilder};
use winit::dpi::LogicalSize;

mod ui;
use ui::{GameState, GAME_HEIGHT, GAME_WIDTH};

mod core;

pub fn run() -> Result<()> {
    let (mut ctx, event_loop) = get_game_context().build()?;

    let my_game = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, my_game);
}

fn get_game_context() -> ContextBuilder {
    let mut cb = ContextBuilder::new("ArenaGS", "Chris Hamons")
        .window_setup(conf::WindowSetup {
            title: "Arena: Gunpowder and Sorcery".to_string(),
            ..Default::default()
        })
        .window_mode(conf::WindowMode {
            logical_size: Some(LogicalSize::new(GAME_WIDTH, GAME_HEIGHT)),
            ..Default::default()
        });

    // Add ArenaGS-Data to the resource path
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let root = path::PathBuf::from(manifest_dir);
        cb = cb.add_resource_path(root.join("..").join("ArenaGS-Data"));
    }

    cb
}
