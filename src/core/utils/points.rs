use std::fmt;

use ggez::{glam::Vec2, mint::Vector2};
use serde::{Deserialize, Serialize};

use crate::core::Map;

use super::Direction;

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

    pub fn in_direction(&self, direction: Direction) -> Option<SizedPoint> {
        let x: i32 = self.origin.x as i32;
        let y: i32 = self.origin.y as i32;
        match direction {
            Direction::North => self.constrain_to_map(x, y - 1),
            Direction::NorthEast => self.constrain_to_map(x + 1, y - 1),
            Direction::East => self.constrain_to_map(x + 1, y),
            Direction::SouthEast => self.constrain_to_map(x + 1, y + 1),
            Direction::South => self.constrain_to_map(x, y + 1),
            Direction::SouthWest => self.constrain_to_map(x - 1, y + 1),
            Direction::West => self.constrain_to_map(x - 1, y),
            Direction::NorthWest => self.constrain_to_map(x - 1, y - 1),
            Direction::None => Some(*self),
        }
    }

    fn constrain_to_map(&self, x: i32, y: i32) -> Option<SizedPoint> {
        let width = self.width as i32;
        let left = x - (width - 1);
        let right = x + (width - 1);

        let height = self.height as i32;
        let top = y - (height - 1);
        let bottom = y + (height - 1);

        if left >= 0 && top >= 0 && bottom < Map::MAX_TILES as i32 && right < Map::MAX_TILES as i32 {
            Some(self.move_to(Point::new(x as u32, y as u32)))
        } else {
            None
        }
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

    #[test]
    fn off_map() {
        assert!(SizedPoint::new(0, 0).in_direction(Direction::North).is_none());
        assert!(SizedPoint::new(12, 0).in_direction(Direction::East).is_none());
        assert!(SizedPoint::new_sized(11, 1, 2, 2).in_direction(Direction::East).is_none());
        assert!(SizedPoint::new_sized(1, 1, 2, 2).in_direction(Direction::North).is_none());
    }

    #[test]
    fn constrained() {
        let point = SizedPoint::new_sized(2, 2, 1, 1);
        assert!(point.constrain_to_map(0, 0).is_some());
        assert!(point.constrain_to_map(12, 12).is_some());
        assert!(point.constrain_to_map(13, 12).is_none());
        assert!(point.constrain_to_map(-1, 0).is_none());
    }

    #[test]
    fn constrained_with_sized() {
        let point = SizedPoint::new_sized(2, 2, 2, 2);
        assert!(point.constrain_to_map(0, 0).is_none());
        assert!(point.constrain_to_map(1, 1).is_some());
        assert!(point.constrain_to_map(11, 11).is_some());
        assert!(point.constrain_to_map(12, 12).is_none());
        assert!(point.constrain_to_map(-1, 0).is_none());
    }
}
