use std::fmt;

use line_drawing::WalkGrid;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
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
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
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
        // TODO - Can we be smarter than checking every point?
        let positions = self.all_positions();
        let shortest = positions.iter().min_by(|first, second| {
            let first = WalkGrid::new((first.x as i32, first.y as i32), (point.x as i32, point.y as i32)).count();
            let second = WalkGrid::new((second.x as i32, second.y as i32), (point.x as i32, point.y as i32)).count();
            first.cmp(&second)
        });
        if let Some(shortest) = shortest {
            Some(
                WalkGrid::new((shortest.x as i32, shortest.y as i32), (point.x as i32, point.y as i32))
                    .map(|(x, y)| Point::init(x as u32, y as u32))
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn distance_to(&self, point: Point) -> Option<u32> {
        if let Some(path) = self.line_to(point) {
            Some(path.len() as u32 - 1) // Path includes both end points
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
    fn distance_to_single() {
        let point = SizedPoint::init(2, 2);
        let distance = point.distance_to(Point::init(4, 5)).unwrap();
        assert_eq!(5, distance);
    }

    #[test]
    fn distance_to_multi() {
        let point = SizedPoint::init_multi(1, 2, 2, 1);
        let distance = point.distance_to(Point::init(4, 5)).unwrap();
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
}
