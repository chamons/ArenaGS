use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{self, PathBuf},
};

use anyhow::Result;
use ggez::{
    event::EventHandler,
    graphics::{self, Color},
    Context, GameResult,
};

pub struct GameState {
    images: HashMap<String, ggez::graphics::Image>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Result<GameState> {
        ctx.gfx.add_font(
            "LibreFranklin-Regular",
            graphics::FontData::from_path(ctx, "/fonts/LibreFranklin-Regular.ttf")?,
        );

        let mut images: HashMap<String, ggez::graphics::Image> = HashMap::new();
        GameState::load_images(ctx, path::PathBuf::from("/"), &mut images)?;

        Ok(GameState { images })
    }

    fn load_images(
        ctx: &mut Context,
        dir: PathBuf,
        images: &mut HashMap<String, ggez::graphics::Image>,
    ) -> Result<()> {
        for item in ctx.fs.read_dir(dir)? {
            if ctx.fs.is_file(&item) {
                if let Some(extension) = item
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(|s| s.to_lowercase())
                {
                    if extension.as_str() == "png" {
                        let image = ggez::graphics::Image::from_path(ctx, &item, false)?;
                        images.insert(item.to_str().unwrap().to_owned(), image);
                    }
                }
            } else {
                GameState::load_images(ctx, item, images)?;
            }
        }
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        canvas.finish(ctx)
    }
}
