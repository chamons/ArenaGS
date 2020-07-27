use specs::prelude::*;
use specs_derive::Component;

use crate::atlas::SizedPoint;

#[derive(Hash, PartialEq, Eq, Component)]
pub struct PositionComponent {
    pub position: SizedPoint,
}

impl PositionComponent {
    pub const fn init(position: SizedPoint) -> PositionComponent {
        PositionComponent { position }
    }
}
