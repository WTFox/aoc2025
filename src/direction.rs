#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Direction {
    pub fn turn(&self, turn: char) -> Self {
        match (self, turn) {
            (Direction::Up, 'R') => Direction::Right,
            (Direction::Right, 'R') => Direction::Down,
            (Direction::Down, 'R') => Direction::Left,
            (Direction::Left, 'R') => Direction::Up,
            (Direction::Up, 'L') => Direction::Left,
            (Direction::Right, 'L') => Direction::Up,
            (Direction::Down, 'L') => Direction::Right,
            (Direction::Left, 'L') => Direction::Down,
            _ => *self,
        }
    }
}
