use specs::prelude::*;

use crate::after_image::RenderCanvas;
use crate::atlas::BoxResult;
use crate::clash::Point;

mod debug;
mod infobar;
mod log;
mod map;
mod skillbar;

#[allow(dead_code)]
#[derive(is_enum_variant, Clone)]
pub enum HitTestResult {
    Skill(String),
    Tile(Point),
    Enemy(Point),
    Field(Point),
}

pub trait View {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()>;
    fn hit_test(&self, _ecs: &World, _x: i32, _y: i32) -> Option<HitTestResult> {
        None
    }
}

pub use debug::DebugView;
pub use infobar::InfoBarView;
pub use log::{LogComponent, LogView};
pub use map::{screen_rect_for_map_grid, screen_to_map_position, MapView, MAP_CORNER_Y, TILE_SIZE};
pub use skillbar::SkillBarView;

// HACK
pub use skillbar::test_skill_name;
