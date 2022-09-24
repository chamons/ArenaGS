#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]

use std::{env, path};

use anyhow::Result;
use ggez::{conf, event, ContextBuilder};

mod ui;
use ui::GameState;

pub mod core;

fn main() -> Result<()> {
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
            width: 2560.0,
            height: 1920.0,
            ..Default::default()
        });

    // Add ArenaGS-Data to the resource path
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let root = path::PathBuf::from(manifest_dir);
        cb = cb.add_resource_path(root.join("..").join("ArenaGS-Data"));
    }

    cb
}
