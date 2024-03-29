use ggez::{
    graphics::{self, Canvas},
    Context,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ScreenCoordinates {
    pub rect: graphics::Rect,
}

impl ScreenCoordinates {
    pub fn new(rect: graphics::Rect) -> ScreenCoordinates {
        ScreenCoordinates { rect }
    }

    // Scaling makes drawing difficult as every operation needs to take the screen scale into effect
    // Instead of that mess, we're going to set virtual coordinates to match the original resolution
    // no scaling.
    pub fn calculate(ctx: &mut Context) -> ScreenCoordinates {
        let window = ctx.gfx.window();
        let inner_size = window.inner_size();
        let scale = window.scale_factor();

        // On macOS we can't request the original resolution as it includes
        // the title bar, and we get squashed images
        let rect = graphics::Rect::new(0.0, 0.0, inner_size.width as f32 / scale as f32, inner_size.height as f32 / scale as f32);

        ScreenCoordinates::new(rect)
    }

    pub fn set_screen(&self, canvas: &mut Canvas) {
        canvas.set_screen_coordinates(self.rect);
    }

    pub fn logical_mouse_position(&self, ctx: &mut ggez::Context, x: f32, y: f32) -> (f32, f32) {
        let screen_rect = self.rect;
        let size = ctx.gfx.window().inner_size();
        let pos_x = (x / (size.width as f32)) * screen_rect.w + screen_rect.x;
        let pos_y = (y / (size.height as f32)) * screen_rect.h + screen_rect.y;
        (pos_x, pos_y)
    }
}
