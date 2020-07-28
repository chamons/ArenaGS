use specs::prelude::*;
use specs_derive::Component;

use crate::atlas::{Point, SizedPoint};

#[derive(Hash, PartialEq, Eq, Component)]
pub struct PositionComponent {
    pub position: SizedPoint,
}

impl PositionComponent {
    pub const fn init(position: SizedPoint) -> PositionComponent {
        PositionComponent { position }
    }

    pub fn move_to(&mut self, point: Point) {
        self.position = self.position.move_to(point);
    }
}
