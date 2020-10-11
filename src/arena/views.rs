use specs::prelude::*;

use crate::after_image::{LayoutChunkIcon, RenderCanvas};
use crate::atlas::prelude::*;
use crate::clash::StatusKind;

mod character_overlay;
mod debug;
mod help_popup;
mod infobar;
mod log;
mod map;
mod skillbar;
mod status_display;
mod text_hittester;
mod text_render_helper;
pub mod view_components;

#[allow(dead_code)]
#[derive(is_enum_variant, Clone, Debug)]
pub enum HitTestResult {
    None,
    Skill(String),
    Tile(Point),
    Enemy(Point),
    Field(Point),
    Orb(Point),
    Icon(LayoutChunkIcon),
    Text(String),
    Status(StatusKind),
    BackButton,
    CloseButton,
    // Consider adding a check in lookup_hittest if we need it to display help
}

pub trait View {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()>;
    fn hit_test(&self, _ecs: &World, _x: i32, _y: i32) -> Option<HitTestResult> {
        None
    }
}

pub use character_overlay::CharacterOverlay;
pub use debug::DebugView;
pub use help_popup::*;
pub use infobar::InfoBarView;
pub use log::{log_event, LogView};
pub use map::{screen_rect_for_map_grid, screen_to_map_position, MapView, MAP_CORNER_Y, TILE_SIZE};
pub use skillbar::{get_current_skill_on_skillbar, get_skill_name_on_skillbar, hotkey_to_skill_index, SkillBarView};
pub use status_display::StatusBarView;
pub use text_hittester::*;
pub use text_render_helper::*;

#[cfg(feature = "image_tester")]
pub mod image_tester;
