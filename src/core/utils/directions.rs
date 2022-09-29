use crate::core::Map;

use super::{Point, SizedPoint};

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
    pub fn from_two_points(initial: &SizedPoint, end: &SizedPoint) -> Direction {
        let x = clamp_to_one(end.origin.x as i32 - initial.origin.x as i32);
        let y = clamp_to_one(end.origin.y as i32 - initial.origin.y as i32);

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

    pub fn point_in_direction(&self, point: &SizedPoint) -> Option<SizedPoint> {
        let x: i32 = point.origin.x as i32;
        let y: i32 = point.origin.y as i32;
        match self {
            Direction::North => constrain_to_map(point, x, y - 1),
            Direction::NorthEast => constrain_to_map(point, x + 1, y - 1),
            Direction::East => constrain_to_map(point, x + 1, y),
            Direction::SouthEast => constrain_to_map(point, x + 1, y + 1),
            Direction::South => constrain_to_map(point, x, y + 1),
            Direction::SouthWest => constrain_to_map(point, x - 1, y + 1),
            Direction::West => constrain_to_map(point, x - 1, y),
            Direction::NorthWest => constrain_to_map(point, x - 1, y - 1),
            Direction::None => Some(*point),
        }
    }
}

fn constrain_to_map(point: &SizedPoint, x: i32, y: i32) -> Option<SizedPoint> {
    let width = point.width as i32;
    let left = x - (width - 1);
    let right = x + (width - 1);

    let height = point.height as i32;
    let top = y - (height - 1);
    let bottom = y + (height - 1);

    if left >= 0 && top >= 0 && bottom < Map::MAX_TILES as i32 && right < Map::MAX_TILES as i32 {
        Some(point.move_to(Point::new(x as u32, y as u32)))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_map() {
        assert!(Direction::North.point_in_direction(&SizedPoint::new(0, 0)).is_none());
        assert!(Direction::East.point_in_direction(&SizedPoint::new(12, 0)).is_none());
        assert!(Direction::East.point_in_direction(&SizedPoint::new_sized(11, 1, 2, 2)).is_none());
        assert!(Direction::North.point_in_direction(&SizedPoint::new_sized(1, 1, 2, 2)).is_none());
    }

    #[test]
    fn constrained() {
        let point = SizedPoint::new_sized(2, 2, 1, 1);
        assert!(constrain_to_map(&point, 0, 0).is_some());
        assert!(constrain_to_map(&point, 12, 12).is_some());
        assert!(constrain_to_map(&point, 13, 12).is_none());
        assert!(constrain_to_map(&point, -1, 0).is_none());
    }

    #[test]
    fn constrained_with_sized() {
        let point = SizedPoint::new_sized(2, 2, 2, 2);
        assert!(constrain_to_map(&point, 0, 0).is_none());
        assert!(constrain_to_map(&point, 1, 1).is_some());
        assert!(constrain_to_map(&point, 11, 11).is_some());
        assert!(constrain_to_map(&point, 12, 12).is_none());
        assert!(constrain_to_map(&point, -1, 0).is_none());
    }

    #[test]
    fn from_two_points() {
        assert_eq!(Direction::North, Direction::from_two_points(&SizedPoint::new(2, 2), &SizedPoint::new(2, 1)));
    }
}
