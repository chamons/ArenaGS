use std::fmt;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub const fn init(x: u32, y: u32) -> Point {
        Point { x, y }
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
        assert_eq!(*all.get(0).unwrap(), Point::init(2, 2));
        assert_eq!(*all.get(1).unwrap(), Point::init(3, 2));
        assert_eq!(*all.get(2).unwrap(), Point::init(2, 1));
        assert_eq!(*all.get(3).unwrap(), Point::init(3, 1));
        assert_eq!(*all.get(4).unwrap(), Point::init(2, 0));
        assert_eq!(*all.get(5).unwrap(), Point::init(3, 0));
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
        assert_eq!(*all.get(0).unwrap(), Point::init(3, 3));
        assert_eq!(*all.get(1).unwrap(), Point::init(4, 3));
        assert_eq!(*all.get(2).unwrap(), Point::init(3, 2));
        assert_eq!(*all.get(3).unwrap(), Point::init(4, 2));
        assert_eq!(*all.get(4).unwrap(), Point::init(3, 1));
        assert_eq!(*all.get(5).unwrap(), Point::init(4, 1));
    }
}
