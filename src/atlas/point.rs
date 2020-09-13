use std::fmt;

use line_drawing::WalkGrid;
use serde::{Deserialize, Serialize};

use super::Direction;

// Points are always in the context of a map, which is a fixed sized
// Negative points and points > 12 are invalid in most contexts
// See ExtendedPoint for an example otherwise
pub const MAX_POINT_SIZE: u32 = 13;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub const fn init(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    pub fn get_burst(&self, distance: u32) -> Vec<Point> {
        let distance = distance as i32;
        let mut points = vec![];
        for i in -distance..=distance {
            for j in -distance..=distance {
                if i.abs() + j.abs() <= distance {
                    let x = i + self.x as i32;
                    let y = j + self.y as i32;
                    if x >= 0 && y >= 0 {
                        points.push(Point::init(x as u32, y as u32));
                    }
                }
            }
        }

        points
    }

    fn get_cone_spread_point(mut p: Point, distance: u32, direction: Direction) -> Option<Point> {
        for _ in 0..distance {
            if let Some(new_point) = direction.point_in_direction(&p) {
                p = new_point;
            } else {
                return None;
            }
        }
        Some(p)
    }

    pub fn get_cone(&self, direction: Direction, distance: u32) -> Vec<Point> {
        let mut points = vec![];
        let mut center = *self;
        let (dir_one, dir_two) = match direction {
            Direction::North | Direction::South => (Direction::West, Direction::East),
            Direction::West | Direction::East => (Direction::North, Direction::South),
            _ => panic!("gen_cone with {:?} direction", direction),
        };

        for i in 0..distance {
            if let Some(next_center) = direction.point_in_direction(&center) {
                center = next_center;
                points.push(center);

                for l in 1..i + 2 {
                    if let Some(first) = Point::get_cone_spread_point(center, l, dir_one) {
                        points.push(first);
                    }
                    if let Some(first) = Point::get_cone_spread_point(center, l, dir_two) {
                        points.push(first);
                    }
                }
            }
        }

        points
    }

    pub fn is_in_bounds(&self) -> bool {
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
        let path: Vec<Point> = path.map(|(x, y)| Point::init(x as u32, y as u32)).collect();
        if path.len() > 0 {
            Some(path)
        } else {
            None
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// Internal point that can point off map safely
struct ExtendedPoint {
    pub x: i32,
    pub y: i32,
}

impl ExtendedPoint {
    const fn init(x: i32, y: i32) -> ExtendedPoint {
        ExtendedPoint { x, y }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct SizedPoint {
    pub origin: Point,
    pub width: u32,
    pub height: u32,
}

impl SizedPoint {
    pub const fn init(x: u32, y: u32) -> SizedPoint {
        SizedPoint {
            origin: Point::init(x, y),
            width: 1,
            height: 1,
        }
    }

    pub const fn from(origin: Point) -> SizedPoint {
        SizedPoint::init(origin.x, origin.y)
    }

    pub const fn init_multi(x: u32, y: u32, width: u32, height: u32) -> SizedPoint {
        SizedPoint {
            origin: Point::init(x, y),
            width,
            height,
        }
    }

    pub fn all_positions(&self) -> Vec<Point> {
        let mut positions = Vec::with_capacity((self.width * self.height) as usize);
        for y in 0..self.height {
            for x in 0..self.width {
                positions.push(Point::init(self.origin.x + x, self.origin.y - y))
            }
        }
        positions
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        self.all_positions().iter().any(|p| *p == *point)
    }

    #[allow(dead_code)]
    pub fn single_position(&self) -> Point {
        assert!(self.width == 1 && self.height == 1);
        self.origin
    }

    #[must_use]
    pub fn move_to(&self, position: Point) -> SizedPoint {
        SizedPoint {
            origin: position,
            width: self.width,
            height: self.height,
        }
    }

    pub fn line_to(&self, point: Point) -> Option<Vec<Point>> {
        self.line_to_extended(ExtendedPoint::init(point.x as i32, point.y as i32))
    }

    fn line_to_extended(&self, point: ExtendedPoint) -> Option<Vec<Point>> {
        // TODO - Can we be smarter than checking every point?
        let positions = self.all_positions();
        let shortest = positions.iter().min_by(|first, second| {
            let first = WalkGrid::new((first.x as i32, first.y as i32), (point.x, point.y)).count();
            let second = WalkGrid::new((second.x as i32, second.y as i32), (point.x, point.y)).count();
            first.cmp(&second)
        });
        if let Some(shortest) = shortest {
            Some(
                WalkGrid::<i32>::new((shortest.x as i32, shortest.y as i32), (point.x as i32, point.y as i32))
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *y < 13 && *x < 13)
                    .map(|(x, y)| Point::init(x as u32, y as u32))
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
        let target_positions = point.all_positions();
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
}

impl fmt::Display for SizedPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{}) {}x{}", self.origin.x, self.origin.y, self.width, self.height)
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
            .line_to_extended(ExtendedPoint::init(ending.origin.x as i32 + delta_x, ending.origin.y as i32 + delta_y))
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
pub fn assert_points_equal(a: Point, b: Point) {
    assert_eq!(a.x, b.x);
    assert_eq!(a.y, b.y);
}

#[cfg(test)]
pub fn assert_points_not_equal(a: Point, b: Point) {
    assert!(a.x != b.x || a.y != b.y);
}

#[cfg(test)]
mod tests {
    use super::super::assert_points_equal;
    use super::*;

    #[test]
    fn all_positions() {
        //  (2,0) (3,0)
        //  (2,1) (3,1)
        //  (2,2) (3,2)
        let point = SizedPoint::init_multi(2, 2, 2, 3);
        let all = point.all_positions();
        assert_eq!(6, all.len());
        assert_eq!(all[0], Point::init(2, 2));
        assert_eq!(all[1], Point::init(3, 2));
        assert_eq!(all[2], Point::init(2, 1));
        assert_eq!(all[3], Point::init(3, 1));
        assert_eq!(all[4], Point::init(2, 0));
        assert_eq!(all[5], Point::init(3, 0));
    }

    #[test]
    fn contains_point() {
        let point = SizedPoint::init_multi(2, 2, 2, 3);
        assert_eq!(true, point.contains_point(&Point::init(2, 2)));
        assert_eq!(true, point.contains_point(&Point::init(3, 2)));
        assert_eq!(true, point.contains_point(&Point::init(2, 1)));
        assert_eq!(true, point.contains_point(&Point::init(3, 1)));
        assert_eq!(true, point.contains_point(&Point::init(2, 0)));
        assert_eq!(true, point.contains_point(&Point::init(3, 0)));
        assert_eq!(false, point.contains_point(&Point::init(4, 4)));
        assert_eq!(false, point.contains_point(&Point::init(0, 0)));
        assert_eq!(false, point.contains_point(&Point::init(2, 5)));
    }

    #[test]
    fn move_by() {
        let mut point = SizedPoint::init_multi(2, 2, 2, 3);
        point = point.move_to(Point::init(3, 3));

        let all = point.all_positions();
        assert_eq!(6, all.len());
        assert_eq!(all[0], Point::init(3, 3));
        assert_eq!(all[1], Point::init(4, 3));
        assert_eq!(all[2], Point::init(3, 2));
        assert_eq!(all[3], Point::init(4, 2));
        assert_eq!(all[4], Point::init(3, 1));
        assert_eq!(all[5], Point::init(4, 1));
    }

    #[test]
    fn line_to_single() {
        let point = SizedPoint::init(2, 2);
        let line = point.line_to(Point::init(4, 5)).unwrap();
        assert_eq!(6, line.len());
        assert_eq!(line[0], Point::init(2, 2));
        assert_eq!(line[1], Point::init(2, 3));
        assert_eq!(line[2], Point::init(3, 3));
        assert_eq!(line[3], Point::init(3, 4));
        assert_eq!(line[4], Point::init(4, 4));
        assert_eq!(line[5], Point::init(4, 5));
    }

    #[test]
    fn line_to_multi() {
        let point = SizedPoint::init_multi(1, 2, 2, 1);
        let line = point.line_to(Point::init(4, 5)).unwrap();
        assert_eq!(6, line.len());
        assert_eq!(line[0], Point::init(2, 2));
        assert_eq!(line[1], Point::init(2, 3));
        assert_eq!(line[2], Point::init(3, 3));
        assert_eq!(line[3], Point::init(3, 4));
        assert_eq!(line[4], Point::init(4, 4));
        assert_eq!(line[5], Point::init(4, 5));
    }

    #[test]
    fn extend_line() {
        let point = SizedPoint::init(2, 2);
        let line = extend_line_along_path(&point.line_to(Point::init(4, 5)).unwrap(), 12);
        assert_eq!(12, line.len());
        assert_eq!(line[0], Point::init(2, 2));
        assert_eq!(line[1], Point::init(2, 3));
        assert_eq!(line[2], Point::init(3, 3));
        assert_eq!(line[3], Point::init(3, 4));
        assert_eq!(line[4], Point::init(4, 4));
        assert_eq!(line[5], Point::init(4, 5));
        assert_eq!(line[6], Point::init(4, 6));
        assert_eq!(line[7], Point::init(5, 6));
        assert_eq!(line[8], Point::init(5, 7));
        assert_eq!(line[9], Point::init(6, 7));
        assert_eq!(line[10], Point::init(6, 8));
        assert_eq!(line[11], Point::init(6, 9));
    }

    #[test]
    fn extend_line_past_map_edge_south() {
        let point = SizedPoint::init(8, 8);
        let line = extend_line_along_path(&point.line_to(Point::init(11, 12)).unwrap(), 12);
        assert_eq!(8, line.len());
        assert_eq!(line[0], Point::init(8, 8));
        assert_eq!(line[1], Point::init(8, 9));
        assert_eq!(line[2], Point::init(9, 9));
        assert_eq!(line[3], Point::init(9, 10));
        assert_eq!(line[4], Point::init(10, 10));
        assert_eq!(line[5], Point::init(10, 11));
        assert_eq!(line[6], Point::init(11, 11));
        assert_eq!(line[7], Point::init(11, 12));
    }

    #[test]
    fn extend_line_past_map_edge_north() {
        let point = SizedPoint::init(3, 3);
        let line = extend_line_along_path(&point.line_to(Point::init(1, 1)).unwrap(), 12);
        assert_eq!(7, line.len());
        assert_eq!(line[0], Point::init(3, 3));
        assert_eq!(line[1], Point::init(3, 2));
        assert_eq!(line[2], Point::init(2, 2));
        assert_eq!(line[3], Point::init(2, 1));
        assert_eq!(line[4], Point::init(1, 1));
        assert_eq!(line[5], Point::init(1, 0));
        assert_eq!(line[6], Point::init(0, 0));
    }

    #[test]
    fn distance_to_single() {
        let point = SizedPoint::init(2, 2);
        let (initial, distance) = point.distance_with_initial(Point::init(4, 5)).unwrap();
        assert_eq!(5, distance);
        assert_points_equal(initial, Point::init(2, 2));
        let distance = point.distance_to(Point::init(4, 5)).unwrap();
        assert_eq!(5, distance);
    }

    #[test]
    fn distance_to_multi() {
        let point = SizedPoint::init_multi(1, 2, 2, 1);
        let (initial, distance) = point.distance_with_initial(Point::init(4, 5)).unwrap();
        assert_eq!(5, distance);
        assert_points_equal(initial, Point::init(2, 2));
        let distance = point.distance_to(Point::init(4, 5)).unwrap();
        assert_eq!(5, distance);
    }

    #[test]
    fn multi_distance_to_multi() {
        let point = SizedPoint::init_multi(1, 2, 2, 2);
        let (initial, end, distance) = point.distance_to_multi_with_endpoints(SizedPoint::init_multi(4, 6, 2, 2)).unwrap();
        // . . . . . .
        // . P P . . .
        // . P P . . .
        // . . * * . .
        // . . . * * .
        // . . . . T T
        // . . . . T T
        assert_eq!(5, distance);
        assert_points_equal(Point::init(2, 2), initial);
        assert_points_equal(Point::init(4, 5), end);
        let distance = point.distance_to_multi(SizedPoint::init_multi(4, 6, 2, 2)).unwrap();
        assert_eq!(5, distance);
    }

    #[test]
    fn burst() {
        let point = Point::init(3, 3);
        assert_eq!(1, point.get_burst(0).len());
        assert_eq!(5, point.get_burst(1).len());
        assert_eq!(13, point.get_burst(2).len());
    }

    #[test]
    fn burst_corner() {
        let point = Point::init(0, 0);
        assert_eq!(3, point.get_burst(1).len());
    }

    #[test]
    fn distance_to_point() {
        assert_eq!(9, Point::init(5, 4).distance_to(Point::init(0, 0)).unwrap());
    }

    #[test]
    fn line_to_point() {
        let path = Point::init(5, 4).line_to(Point::init(0, 0)).unwrap();
        assert_points_equal(path[0], Point::init(5, 4));
        assert_points_equal(path[4], Point::init(3, 2));
        assert_points_equal(path[9], Point::init(0, 0));
    }

    #[test]
    fn cone() {
        let point = Point::init(3, 3);
        let points = point.get_cone(Direction::North, 2);
        assert_eq!(8, points.len());
        assert!(points.contains(&Point::init(3, 2)));
        assert!(points.contains(&Point::init(2, 2)));
        assert!(points.contains(&Point::init(4, 2)));
        assert!(points.contains(&Point::init(3, 1)));
        assert!(points.contains(&Point::init(2, 1)));
        assert!(points.contains(&Point::init(4, 1)));
        assert!(points.contains(&Point::init(1, 1)));
        assert!(points.contains(&Point::init(5, 1)));
    }

    #[test]
    fn cone_corner() {
        let point = Point::init(1, 4);
        let points = point.get_cone(Direction::West, 3);
        assert_eq!(3, points.len());
        assert!(points.contains(&Point::init(0, 4)));
        assert!(points.contains(&Point::init(0, 3)));
        assert!(points.contains(&Point::init(0, 5)));
    }
}
