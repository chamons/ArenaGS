use specs::prelude::*;
use specs_derive::Component;

use crate::clash::Point;

#[derive(Hash, PartialEq, Eq, Component)]
pub struct PositionComponent {
    pub origin: Point,
    pub width: u32,
    pub height: u32,
}

impl PositionComponent {
    pub const fn init(x: u32, y: u32) -> PositionComponent {
        PositionComponent {
            origin: Point::init(x, y),
            width: 1,
            height: 1,
        }
    }
    pub const fn init_multi(x: u32, y: u32, width: u32, height: u32) -> PositionComponent {
        PositionComponent {
            origin: Point::init(x, y),
            width,
            height,
        }
    }

    pub fn all_positions(&self) -> Vec<Point> {
        let mut positions = Vec::with_capacity((self.width * self.height) as usize);
        for y in 0..self.height {
            for x in 0..self.width {
                positions.push(Point::init(self.origin.x + x, self.origin.y + y))
            }
        }
        positions
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        self.all_positions().iter().any(|p| *p == *point)
    }

    pub fn single_position(&self) -> Point {
        assert!(self.width == 1 && self.height == 1);
        self.origin
    }

    pub fn move_to(&mut self, position: Point) {
        self.origin = position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_positions() {
        //  (2,4) (3,4)
        //  (2,3) (3,3)
        //  (2,2) (3,2)
        // Figure out why you click upper left doesn't count as enemy for hittest
        let position_component = PositionComponent::init_multi(2, 2, 2, 3);
        let all = position_component.all_positions();
        assert_eq!(6, all.len());
        assert_eq!(*all.get(0).unwrap(), Point::init(2, 2));
        assert_eq!(*all.get(1).unwrap(), Point::init(3, 2));
        assert_eq!(*all.get(2).unwrap(), Point::init(2, 3));
        assert_eq!(*all.get(3).unwrap(), Point::init(3, 3));
        assert_eq!(*all.get(4).unwrap(), Point::init(2, 4));
        assert_eq!(*all.get(5).unwrap(), Point::init(3, 4));
    }

    #[test]
    fn contains_point() {
        let position_component = PositionComponent::init_multi(2, 2, 2, 3);
        assert_eq!(true, position_component.contains_point(&Point::init(2, 2)));
        assert_eq!(true, position_component.contains_point(&Point::init(3, 2)));
        assert_eq!(true, position_component.contains_point(&Point::init(2, 3)));
        assert_eq!(true, position_component.contains_point(&Point::init(3, 3)));
        assert_eq!(true, position_component.contains_point(&Point::init(2, 2)));
        assert_eq!(true, position_component.contains_point(&Point::init(3, 4)));
        assert_eq!(false, position_component.contains_point(&Point::init(4, 4)));
        assert_eq!(false, position_component.contains_point(&Point::init(0, 0)));
        assert_eq!(false, position_component.contains_point(&Point::init(2, 5)));
    }

    #[test]
    fn move_by() {
        let mut position_component = PositionComponent::init_multi(2, 2, 2, 3);
        position_component.move_to(Point::init(3, 3));

        let all = position_component.all_positions();
        assert_eq!(6, all.len());
        assert_eq!(*all.get(0).unwrap(), Point::init(3, 3));
        assert_eq!(*all.get(1).unwrap(), Point::init(4, 3));
        assert_eq!(*all.get(2).unwrap(), Point::init(3, 4));
        assert_eq!(*all.get(3).unwrap(), Point::init(4, 4));
        assert_eq!(*all.get(4).unwrap(), Point::init(3, 5));
        assert_eq!(*all.get(5).unwrap(), Point::init(4, 5));
    }
}
