use ggez::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ScreenScale {
    pub scale: f64,
}

impl ScreenScale {
    pub fn new(ctx: &mut Context) -> Self {
        ScreenScale {
            scale: ctx.gfx.window().scale_factor(),
        }
    }
}