use crate::Direction;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new() -> Point {
        Point::default()
    }

    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn distance_from(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        }
    }

    pub fn moved(&self, direction: Direction) -> Self {
        let mut new_point = *self;
        match direction {
            Direction::Up => new_point.y -= 1,
            Direction::Down => new_point.y += 1,
            Direction::Left => new_point.x -= 1,
            Direction::Right => new_point.x += 1,
        }
        new_point
    }
}
