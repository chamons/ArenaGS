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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn covered_points() {
        //  (2,0) (3,0)
        //  (2,1) (3,1)
        //  (2,2) (3,2)
        let point = SizedPoint::new_sized(2, 2, 2, 3);
        let all = point.covered_points();
        assert_eq!(6, all.len());
        assert_eq!(all[0], Point::new(2, 2));
        assert_eq!(all[1], Point::new(3, 2));
        assert_eq!(all[2], Point::new(2, 1));
        assert_eq!(all[3], Point::new(3, 1));
        assert_eq!(all[4], Point::new(2, 0));
        assert_eq!(all[5], Point::new(3, 0));
    }

    #[test]
    fn contains_point() {
        let point = SizedPoint::new_sized(2, 2, 2, 3);
        assert!(point.contains_point(&Point::new(2, 2)));
        assert!(point.contains_point(&Point::new(3, 2)));
        assert!(point.contains_point(&Point::new(2, 1)));
        assert!(point.contains_point(&Point::new(3, 1)));
        assert!(point.contains_point(&Point::new(2, 0)));
        assert!(point.contains_point(&Point::new(3, 0)));
        assert!(!point.contains_point(&Point::new(4, 4)));
        assert!(!point.contains_point(&Point::new(0, 0)));
        assert!(!point.contains_point(&Point::new(2, 5)));
    }

    #[test]
    fn move_by() {
        let mut point = SizedPoint::new_sized(2, 2, 2, 3);
        point = point.move_to(Point::new(3, 3));

        let all = point.covered_points();
        assert_eq!(6, all.len());
        assert_eq!(all[0], Point::new(3, 3));
        assert_eq!(all[1], Point::new(4, 3));
        assert_eq!(all[2], Point::new(3, 2));
        assert_eq!(all[3], Point::new(4, 2));
        assert_eq!(all[4], Point::new(3, 1));
        assert_eq!(all[5], Point::new(4, 1));
    }
}
