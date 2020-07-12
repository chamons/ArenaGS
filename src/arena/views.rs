use specs::prelude::*;

use crate::after_image::RenderCanvas;
use crate::atlas::BoxResult;

mod map;

mod skillbar;

pub trait View {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()>;
}

pub use map::{MapView, TILE_SIZE};
pub use skillbar::SkillBarView;
