use specs::prelude::*;

use sdl2::mouse::MouseButton;

use crate::after_image::{LayoutChunkIcon, RenderCanvas};
use crate::atlas::prelude::*;
use crate::clash::StatusKind;

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
    Button,
    // Consider adding a check in lookup_hittest if we need it to display help
}

pub trait View {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()>;
    fn hit_test(&self, _ecs: &World, _x: i32, _y: i32) -> Option<HitTestResult> {
        None
    }
    fn handle_mouse(&mut self, _ecs: &World, _x: i32, _y: i32, _button: Option<MouseButton>) {}
}

mod views;
pub use views::*;