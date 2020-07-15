use specs::prelude::*;

use crate::after_image::RenderCanvas;
use crate::atlas::BoxResult;

mod infobar;
mod log;
mod map;
mod skillbar;

pub trait View {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()>;
    fn get_tooltip(&self, _ecs: &World, _x: i32, _y: i32) -> Option<String> {
        None
    }
}

pub use infobar::InfoBarView;
pub use log::{LogComponent, LogView};
pub use map::{MapView, MAP_CORNER_Y, TILE_SIZE};
pub use skillbar::SkillBarView;
