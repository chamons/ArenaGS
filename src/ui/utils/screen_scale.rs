use ggez::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ScreenScale {
    pub scale: f32,
}

impl ScreenScale {
    pub fn new(ctx: &mut Context) -> Self {
        ScreenScale {
            scale: 1.0, //ctx.gfx.window().scale_factor() as f32,
        }
    }
}
