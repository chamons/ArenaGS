use specs::prelude::*;
use specs_derive::*;

use crate::atlas::prelude::*;

#[derive(Component, Clone)] // NotConvertSaveload
pub struct MousePositionComponent {
    pub position: Point,
}

impl MousePositionComponent {
    pub fn init() -> MousePositionComponent {
        MousePositionComponent { position: Point::init(0, 0) }
    }
}
