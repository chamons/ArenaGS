use crate::atlas::Point;

#[derive(Clone, Copy)]
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

    pub fn from_delta(delta_x: i32, delta_y: i32) -> Direction {
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

    pub fn point_in_direction(&self, point: Point) -> Point {
        match self {
            Direction::North => Point::init(point.x, point.y - 1),
            Direction::NorthEast => Point::init(point.x + 1, point.y - 1),
            Direction::East => Point::init(point.x + 1, point.y),
            Direction::SouthEast => Point::init(point.x + 1, point.y + 1),
            Direction::South => Point::init(point.x, point.y + 1),
            Direction::SouthWest => Point::init(point.x - 1, point.y + 1),
            Direction::West => Point::init(point.x - 1, point.y),
            Direction::NorthWest => Point::init(point.x - 1, point.y - 1),
            Direction::None => point,
        }
    }
}
