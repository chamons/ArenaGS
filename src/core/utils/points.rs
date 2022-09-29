use std::fmt;

use ggez::glam::Vec2;
use serde::{Deserialize, Serialize};

// Points are always in the context of a map, which is a fixed sized
// Negative points and points > 12 are invalid in most contexts
pub const MAX_POINT_SIZE: u32 = 13;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub const fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    #[allow(dead_code)]
    pub fn in_bounds(&self) -> bool {
        self.x < MAX_POINT_SIZE && self.y < MAX_POINT_SIZE
    }
}

impl Into<Vec2> for Point {
    fn into(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct SizedPoint {
    pub origin: Point,
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)]
impl SizedPoint {
    pub const fn new(x: u32, y: u32) -> SizedPoint {
        SizedPoint {
            origin: Point::new(x, y),
            width: 1,
            height: 1,
        }
    }

    pub const fn new_sized(x: u32, y: u32, width: u32, height: u32) -> SizedPoint {
        SizedPoint {
            origin: Point::new(x, y),
            width,
            height,
        }
    }

    pub fn covered_points(&self) -> Vec<Point> {
        let mut positions = Vec::with_capacity((self.width * self.height) as usize);
        for y in 0..self.height {
            for x in 0..self.width {
                positions.push(Point::new(self.origin.x + x, self.origin.y - y))
            }
        }
        positions
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        self.covered_points().iter().any(|p| *p == *point)
    }

    #[must_use]
    pub fn move_to(&self, position: Point) -> SizedPoint {
        SizedPoint {
            origin: position,
            width: self.width,
            height: self.height,
        }
    }
}

impl From<Point> for SizedPoint {
    fn from(origin: Point) -> Self {
        SizedPoint::new(origin.x, origin.y)
    }
}

impl fmt::Display for SizedPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{}) {}x{}", self.origin.x, self.origin.y, self.width, self.height)
    }
}
