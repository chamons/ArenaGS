use ggez::{
    event::EventHandler,
    graphics::{self, Color},
    Context, GameResult,
};

pub struct GameState {}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        GameState {}
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
