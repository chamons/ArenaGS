use crate::atlas::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    None,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn clamp_to_one(x: i32) -> i32 {
    if x > 1 {
        1
    } else if x < -1 {
        -1
    } else {
        x
    }
}

impl Direction {
    pub fn from_two_points(initial: &Point, end: &Point) -> Direction {
        let x = clamp_to_one(end.x as i32 - initial.x as i32);
        let y = clamp_to_one(end.y as i32 - initial.y as i32);

        Direction::from_delta(x, y)
    }

    fn from_delta(delta_x: i32, delta_y: i32) -> Direction {
        if delta_x == 1 {
            if delta_y == 1 {
                Direction::SouthEast
            } else if delta_y == -1 {
                Direction::NorthEast
            } else {
                Direction::East
            }
        } else if delta_x == -1 {
            if delta_y == 1 {
                Direction::SouthWest
            } else if delta_y == -1 {
                Direction::NorthWest
            } else {
                Direction::West
            }
        } else {
            if delta_y == 1 {
                Direction::South
            } else if delta_y == -1 {
                Direction::North
            } else {
                Direction::None
            }
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
            Direction::None => Direction::None,
        }
    }

    pub fn point_in_direction(&self, point: &Point) -> Option<Point> {
        let x: i32 = point.x as i32;
        let y: i32 = point.y as i32;
        match self {
            Direction::North => constrained_point(x, y - 1),
            Direction::NorthEast => constrained_point(x + 1, y - 1),
            Direction::East => constrained_point(x + 1, y),
            Direction::SouthEast => constrained_point(x + 1, y + 1),
            Direction::South => constrained_point(x, y + 1),
            Direction::SouthWest => constrained_point(x - 1, y + 1),
            Direction::West => constrained_point(x - 1, y),
            Direction::NorthWest => constrained_point(x - 1, y - 1),
            Direction::None => Some(*point),
        }
    }

    pub fn sized_point_in_direction(&self, point: &SizedPoint) -> Option<SizedPoint> {
        if let Some(new_origin) = self.point_in_direction(&point.origin) {
            Some(SizedPoint::init_multi(new_origin.x, new_origin.y, point.width, point.height))
        } else {
            None
        }
    }
}

use crate::clash::MAX_MAP_TILES;

fn constrained_point(x: i32, y: i32) -> Option<Point> {
    if x >= 0 && y >= 0 && y < MAX_MAP_TILES as i32 && x < MAX_MAP_TILES as i32 {
        Some(Point::init(x as u32, y as u32))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_map() {
        assert!(Direction::North.point_in_direction(&Point::init(0, 0)).is_none());
        assert!(Direction::North.sized_point_in_direction(&SizedPoint::init(0, 0)).is_none());
    }

    #[test]
    fn constrained() {
        assert!(constrained_point(0, 0).is_some());
        assert!(constrained_point(12, 12).is_some());
        assert!(constrained_point(13, 12).is_none());
        assert!(constrained_point(-1, 0).is_none());
    }

    #[test]
    fn from_two_points() {
        assert_eq!(Direction::North, Direction::from_two_points(&Point::init(2, 2), &Point::init(2, 1)));
    }
}
