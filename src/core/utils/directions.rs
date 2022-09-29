use super::SizedPoint;

#[allow(dead_code)]
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

#[allow(dead_code)]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_two_points() {
        assert_eq!(Direction::North, Direction::from_two_points(&SizedPoint::new(2, 2), &SizedPoint::new(2, 1)));
    }

    #[test]
    fn opposite() {
        assert_eq!(Direction::North, Direction::South.opposite());
    }
}
