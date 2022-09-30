use std::fmt;

use ggez::{glam::Vec2, mint::Vector2};
use line_drawing::WalkGrid;
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

    pub fn distance_to(&self, point: Point) -> Option<u32> {
        if let Some(path) = self.line_to(point) {
            Some(path.len() as u32 - 1) // Path includes both end points
        } else {
            None
        }
    }

    pub fn line_to(&self, point: Point) -> Option<Vec<Point>> {
        let path = WalkGrid::new((self.x as i32, self.y as i32), (point.x as i32, point.y as i32));
        let path: Vec<Point> = path.map(|(x, y)| Point::new(x as u32, y as u32)).collect();
        if path.len() > 0 {
            Some(path)
        } else {
            None
        }
    }
}

impl From<Point> for Vec2 {
    fn from(s: Point) -> Vec2 {
        Vec2::new(s.x as f32, s.y as f32)
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

    pub fn line_to(&self, point: Point) -> Option<Vec<Point>> {
        self.line_to_extended(Point::new(point.x, point.y))
    }

    fn line_to_extended(&self, point: Point) -> Option<Vec<Point>> {
        // TODO - Can we be smarter than checking every point?
        let positions = self.covered_points();
        let shortest = positions.iter().min_by(|first, second| {
            let first = WalkGrid::new((first.x as i32, first.y as i32), (point.x as i32, point.y as i32)).count();
            let second = WalkGrid::new((second.x as i32, second.y as i32), (point.x as i32, point.y as i32)).count();
            first.cmp(&second)
        });
        if let Some(shortest) = shortest {
            Some(
                WalkGrid::<i32>::new((shortest.x as i32, shortest.y as i32), (point.x as i32, point.y as i32))
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *y < 13 && *x < 13)
                    .map(|(x, y)| Point::new(x as u32, y as u32))
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn distance_with_initial(&self, point: Point) -> Option<(Point, u32)> {
        if let Some(path) = self.line_to(point) {
            Some((*path.first().unwrap(), path.len() as u32 - 1)) // Path includes both end points
        } else {
            None
        }
    }

    pub fn distance_to(&self, point: Point) -> Option<u32> {
        if let Some((_, first)) = self.distance_with_initial(point) {
            Some(first)
        } else {
            None
        }
    }

    pub fn distance_to_multi_with_endpoints(&self, point: SizedPoint) -> Option<(Point, Point, u32)> {
        // TODO - Can we be smarter than checking every point?
        let target_positions = point.covered_points();
        let shortest_target = target_positions.iter().min_by(|first, second| {
            let first = self.distance_to(**first);
            let second = self.distance_to(**second);
            first.cmp(&second)
        });

        if let Some(shortest_target) = shortest_target {
            if let Some((source, distance)) = self.distance_with_initial(*shortest_target) {
                Some((source, *shortest_target, distance))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn distance_to_multi(&self, point: SizedPoint) -> Option<u32> {
        if let Some((_, _, first)) = self.distance_to_multi_with_endpoints(point) {
            Some(first)
        } else {
            None
        }
    }

    pub fn nearest_point_to(&self, point: SizedPoint) -> Point {
        *self
            .covered_points()
            .iter()
            .min_by(|&&x, &&y| point.distance_to(x).cmp(&point.distance_to(y)))
            .take()
            .unwrap()
    }

    pub fn visual_center(&self) -> Vector2<f32> {
        Vector2 {
            x: self.origin.x as f32 + (self.width as f32 / 2.0),
            y: self.origin.y as f32 + (self.height as f32 / 2.0),
        }
    }
}

impl fmt::Display for SizedPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{}) {}x{}", self.origin.x, self.origin.y, self.width, self.height)
    }
}

impl From<Point> for SizedPoint {
    fn from(p: Point) -> Self {
        SizedPoint::new(p.x, p.y)
    }
}

pub fn extend_line_along_path(points: &[Point], length: u32) -> Vec<Point> {
    let starting = points.first().unwrap();
    let ending = points.last().unwrap();

    let delta_x: i32 = ending.x as i32 - starting.x as i32;
    let delta_y: i32 = ending.y as i32 - starting.y as i32;

    let mut line = points.to_vec();
    loop {
        let ending = SizedPoint::from(*line.last().unwrap());
        let extension = ending
            .line_to_extended(Point::new((ending.origin.x as i32 + delta_x) as u32, (ending.origin.y as i32 + delta_y) as u32))
            .unwrap();
        for e in extension.iter().skip(1) {
            line.push(*e);
        }

        // If we've reached our length or the extension adds no additional length due to map edge
        if line.len() >= length as usize || extension.len() == 1 {
            line.truncate(length as usize);
            return line;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;

    pub fn assert_points_equal(a: Point, b: Point) {
        assert_eq!(a.x, b.x);
        assert_eq!(a.y, b.y);
    }

    pub fn assert_points_not_equal(a: Point, b: Point) {
        assert!(a.x != b.x || a.y != b.y);
    }

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

    #[test]
    fn line_to_single() {
        let point = SizedPoint::new(2, 2);
        let line = point.line_to(Point::new(4, 5)).unwrap();
        assert_eq!(6, line.len());
        assert_eq!(line[0], Point::new(2, 2));
        assert_eq!(line[1], Point::new(2, 3));
        assert_eq!(line[2], Point::new(3, 3));
        assert_eq!(line[3], Point::new(3, 4));
        assert_eq!(line[4], Point::new(4, 4));
        assert_eq!(line[5], Point::new(4, 5));
    }

    #[test]
    fn line_to_multi() {
        let point = SizedPoint::new_sized(1, 2, 2, 1);
        let line = point.line_to(Point::new(4, 5)).unwrap();
        assert_eq!(6, line.len());
        assert_eq!(line[0], Point::new(2, 2));
        assert_eq!(line[1], Point::new(2, 3));
        assert_eq!(line[2], Point::new(3, 3));
        assert_eq!(line[3], Point::new(3, 4));
        assert_eq!(line[4], Point::new(4, 4));
        assert_eq!(line[5], Point::new(4, 5));
    }

    #[test]
    fn extend_line() {
        let point = SizedPoint::new(2, 2);
        let line = extend_line_along_path(&point.line_to(Point::new(4, 5)).unwrap(), 12);
        assert_eq!(12, line.len());
        assert_eq!(line[0], Point::new(2, 2));
        assert_eq!(line[1], Point::new(2, 3));
        assert_eq!(line[2], Point::new(3, 3));
        assert_eq!(line[3], Point::new(3, 4));
        assert_eq!(line[4], Point::new(4, 4));
        assert_eq!(line[5], Point::new(4, 5));
        assert_eq!(line[6], Point::new(4, 6));
        assert_eq!(line[7], Point::new(5, 6));
        assert_eq!(line[8], Point::new(5, 7));
        assert_eq!(line[9], Point::new(6, 7));
        assert_eq!(line[10], Point::new(6, 8));
        assert_eq!(line[11], Point::new(6, 9));
    }

    #[test]
    fn extend_line_past_map_edge_south() {
        let point = SizedPoint::new(8, 8);
        let line = extend_line_along_path(&point.line_to(Point::new(11, 12)).unwrap(), 12);
        assert_eq!(8, line.len());
        assert_eq!(line[0], Point::new(8, 8));
        assert_eq!(line[1], Point::new(8, 9));
        assert_eq!(line[2], Point::new(9, 9));
        assert_eq!(line[3], Point::new(9, 10));
        assert_eq!(line[4], Point::new(10, 10));
        assert_eq!(line[5], Point::new(10, 11));
        assert_eq!(line[6], Point::new(11, 11));
        assert_eq!(line[7], Point::new(11, 12));
    }

    #[test]
    fn extend_line_past_map_edge_north() {
        let point = SizedPoint::new(3, 3);
        let line = extend_line_along_path(&point.line_to(Point::new(1, 1)).unwrap(), 12);
        assert_eq!(7, line.len());
        assert_eq!(line[0], Point::new(3, 3));
        assert_eq!(line[1], Point::new(3, 2));
        assert_eq!(line[2], Point::new(2, 2));
        assert_eq!(line[3], Point::new(2, 1));
        assert_eq!(line[4], Point::new(1, 1));
        assert_eq!(line[5], Point::new(1, 0));
        assert_eq!(line[6], Point::new(0, 0));
    }

    #[test]
    fn distance_to_single() {
        let point = SizedPoint::new(2, 2);
        let (initial, distance) = point.distance_with_initial(Point::new(4, 5)).unwrap();
        assert_eq!(5, distance);
        assert_points_equal(initial, Point::new(2, 2));
        let distance = point.distance_to(Point::new(4, 5)).unwrap();
        assert_eq!(5, distance);
    }

    #[test]
    fn distance_to_multi() {
        let point = SizedPoint::new_sized(1, 2, 2, 1);
        let (initial, distance) = point.distance_with_initial(Point::new(4, 5)).unwrap();
        assert_eq!(5, distance);
        assert_points_equal(initial, Point::new(2, 2));
        let distance = point.distance_to(Point::new(4, 5)).unwrap();
        assert_eq!(5, distance);
    }

    #[test]
    fn multi_distance_to_multi() {
        let point = SizedPoint::new_sized(1, 2, 2, 2);
        let (initial, end, distance) = point.distance_to_multi_with_endpoints(SizedPoint::new_sized(4, 6, 2, 2)).unwrap();
        // . . . . . .
        // . P P . . .
        // . P P . . .
        // . . * * . .
        // . . . * * .
        // . . . . T T
        // . . . . T T
        assert_eq!(5, distance);
        assert_points_equal(Point::new(2, 2), initial);
        assert_points_equal(Point::new(4, 5), end);
        let distance = point.distance_to_multi(SizedPoint::new_sized(4, 6, 2, 2)).unwrap();
        assert_eq!(5, distance);
    }

    #[test]
    fn distance_to_point() {
        assert_eq!(9, Point::new(5, 4).distance_to(Point::new(0, 0)).unwrap());
    }

    #[test]
    fn line_to_point() {
        let path = Point::new(5, 4).line_to(Point::new(0, 0)).unwrap();
        assert_points_equal(path[0], Point::new(5, 4));
        assert_points_equal(path[4], Point::new(3, 2));
        assert_points_equal(path[9], Point::new(0, 0));
    }

    #[test]
    fn nearest_to_multi_point() {
        let point = SizedPoint::new(2, 2);
        let target = SizedPoint::new_sized(3, 3, 2, 2);
        assert_points_equal(target.nearest_point_to(point), Point::new(3, 2));
    }

    #[test]
    fn visual_center() {
        let point = SizedPoint::new(2, 3);
        let center = point.visual_center();
        assert_approx_eq!(2.5, center.x);
        assert_approx_eq!(3.5, center.y);

        let point = SizedPoint::new_sized(2, 3, 2, 2);
        let center = point.visual_center();
        assert_approx_eq!(3.0, center.x);
        assert_approx_eq!(4.0, center.y);
    }
}
