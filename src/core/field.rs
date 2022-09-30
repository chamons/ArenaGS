use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

use super::Point;

#[derive(Debug, Deserialize, Serialize)]
pub enum FieldColor {
    Gray,
}

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Fields {
    pub color: FieldColor,
    pub positions: Vec<Point>,
}

impl Fields {
    pub fn new(color: FieldColor, positions: &[Point]) -> Self {
        Fields {
            color,
            positions: Vec::from(positions),
        }
    }
}
