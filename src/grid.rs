use std::fmt;
use std::ops::{Add, AddAssign, Mul, Sub};

// ============================================================================
// Direction
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    pub fn delta(self) -> Point {
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }

    pub fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '^' | 'U' | 'N' => Some(Direction::Up),
            'v' | 'D' | 'S' => Some(Direction::Down),
            '<' | 'L' | 'W' => Some(Direction::Left),
            '>' | 'R' | 'E' => Some(Direction::Right),
            _ => None,
        }
    }
}

// ============================================================================
// Point
// ============================================================================

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub const ZERO: Point = Point { x: 0, y: 0 };

    pub const fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    pub fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    pub fn manhattan_to(&self, other: Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighbors4(&self) -> [Point; 4] {
        Direction::ALL.map(|d| *self + d.delta())
    }

    pub fn neighbors8(&self) -> [Point; 8] {
        [
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
        ]
    }

    pub fn step(self, dir: Direction) -> Self {
        self + dir.delta()
    }

    pub fn in_bounds(&self, width: i64, height: i64) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<i64> for Point {
    type Output = Self;
    fn mul(self, scalar: i64) -> Self {
        Point::new(self.x * scalar, self.y * scalar)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ============================================================================
// Grid
// ============================================================================

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Grid {
            width,
            height,
            cells: vec![default; width * height],
        }
    }

    pub fn fill(&mut self, value: T) {
        self.cells.fill(value);
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn new_default(width: usize, height: usize) -> Self {
        Grid::new(width, height, T::default())
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn in_bounds(&self, p: Point) -> bool {
        p.x >= 0 && p.y >= 0 && (p.x as usize) < self.width && (p.y as usize) < self.height
    }

    pub fn get(&self, p: Point) -> Option<&T> {
        if self.in_bounds(p) {
            Some(&self.cells[self.index(p.x as usize, p.y as usize)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        if self.in_bounds(p) {
            let i = self.index(p.x as usize, p.y as usize);
            Some(&mut self.cells[i])
        } else {
            None
        }
    }

    pub fn set(&mut self, p: Point, value: T) {
        if self.in_bounds(p) {
            let i = self.index(p.x as usize, p.y as usize);
            self.cells[i] = value;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point, &T)> {
        self.cells.iter().enumerate().map(|(i, v)| {
            let x = (i % self.width) as i64;
            let y = (i / self.width) as i64;
            (Point::new(x, y), v)
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Point, &mut T)> {
        let width = self.width;
        self.cells.iter_mut().enumerate().map(move |(i, v)| {
            let x = (i % width) as i64;
            let y = (i / width) as i64;
            (Point::new(x, y), v)
        })
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        let width = self.width as i64;
        let height = self.height as i64;
        (0..height).flat_map(move |y| (0..width).map(move |x| Point::new(x, y)))
    }

    pub fn find<F>(&self, predicate: F) -> Option<Point>
    where
        F: Fn(&T) -> bool,
    {
        self.iter().find(|(_, v)| predicate(v)).map(|(p, _)| p)
    }

    pub fn find_all<F>(&self, predicate: F) -> Vec<Point>
    where
        F: Fn(&T) -> bool,
    {
        self.iter()
            .filter(|(_, v)| predicate(v))
            .map(|(p, _)| p)
            .collect()
    }

    pub fn count<F>(&self, predicate: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        self.cells.iter().filter(|v| predicate(v)).count()
    }

    /// Very similar to Point.neighbors4, but this returns only valid Points in bounds.
    pub fn neighbors4(&self, p: Point) -> impl Iterator<Item = Point> {
        let width = self.width as i64;
        let height = self.height as i64;
        p.neighbors4()
            .into_iter()
            .filter(move |n| n.in_bounds(width, height))
    }

    /// Very similar to Point.neighbors8, but this returns only valid Points in bounds.
    pub fn neighbors8(&self, p: Point) -> impl Iterator<Item = Point> {
        let width = self.width as i64;
        let height = self.height as i64;
        p.neighbors8()
            .into_iter()
            .filter(move |n| n.in_bounds(width, height))
    }
}

impl Grid<char> {
    pub fn from_str(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.first().map(|l| l.len()).unwrap_or(0);
        let cells: Vec<char> = lines.iter().flat_map(|l| l.chars()).collect();

        Grid {
            width,
            height,
            cells,
        }
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find_value(&self, target: &T) -> Option<Point> {
        self.find(|v| v == target)
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.cells[self.index(x, y)])?;
            }
            if y < self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid {}x{}", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{:?} ", self.cells[self.index(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(rows: Vec<Vec<T>>) -> Self {
        let height = rows.len();
        let width = rows.first().map(|r| r.len()).unwrap_or(0);
        let cells = rows.into_iter().flatten().collect();

        Grid {
            width,
            height,
            cells,
        }
    }
}
// ============================================================================
// Usage examples
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_grid() {
        let input = "##..\n#..#\n....";
        let grid = Grid::from_str(input);

        assert_eq!(grid.width(), 4);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.get(Point::new(0, 0)), Some(&'#'));
        assert_eq!(grid.get(Point::new(2, 0)), Some(&'.'));
    }

    #[test]
    fn find_start() {
        let input = ".....\n..S..\n.....";
        let grid = Grid::from_str(input);
        let start = grid.find_value(&'S');

        assert_eq!(start, Some(Point::new(2, 1)));
    }

    #[test]
    fn movement() {
        let pos = Point::new(5, 5);
        assert_eq!(pos.step(Direction::Up), Point::new(5, 4));
        assert_eq!(pos.step(Direction::Right), Point::new(6, 5));
    }

    #[test]
    fn neighbors() {
        let grid: Grid<char> = Grid::from_str("...\n.X.\n...");
        let center = Point::new(1, 1);

        let valid_neighbors: Vec<_> = center
            .neighbors4()
            .into_iter()
            .filter(|p| grid.in_bounds(*p))
            .collect();

        assert_eq!(valid_neighbors.len(), 4);
    }

    #[test]
    fn bfs_pattern() {
        use std::collections::{HashSet, VecDeque};

        let input = "#####\n#...#\n#.#.#\n#...#\n#####";
        let grid = Grid::from_str(input);
        let start = Point::new(1, 1);

        let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<(Point, i32)> = VecDeque::new();

        queue.push_back((start, 0));
        visited.insert(start);

        let mut max_dist = 0;
        while let Some((pos, dist)) = queue.pop_front() {
            max_dist = max_dist.max(dist);

            for next in pos.neighbors4() {
                if grid.get(next) == Some(&'.') && !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((next, dist + 1));
                }
            }
        }

        assert_eq!(max_dist, 4);
    }
}
