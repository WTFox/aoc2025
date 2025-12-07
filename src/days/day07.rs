use crate::{Direction, Grid, Point};
use itertools::Itertools;

pub fn part_one(input: &str) -> isize {
    let grid = Grid::from_str(input.trim());
    let start = grid.find_value(&'S').expect("no starting position found");

    let mut split_count = 0;
    let mut beams = vec![start];
    for _ in start.y..grid.height as i64 {
        let mut new_beams = Vec::new();

        for b in beams {
            if !grid.in_bounds(b) {
                continue;
            }

            let moved = b.step(Direction::Down);
            if let Some(c) = grid.get(moved) {
                match c {
                    '.' => {
                        new_beams.push(moved);
                    }
                    '^' => {
                        split_count += 1;
                        new_beams.push(Point::new(moved.x - 1, moved.y));
                        new_beams.push(Point::new(moved.x + 1, moved.y));
                    }
                    _ => unreachable!(),
                }
            }
        }
        beams = new_beams
            .iter()
            .unique()
            .map(|&p| p)
            .collect::<Vec<_>>()
            .clone();
    }
    split_count
}

pub fn part_two(input: &str) -> usize {
    let grid = Grid::from_str(input.trim());
    let start = grid.find_value(&'S').expect("no starting position found");

    let mut beams = vec![start];

    let mut split_count = 1;

    for _ in start.y..grid.height as i64 {
        let mut new_beams = Vec::new();
        for b in beams {
            if !grid.in_bounds(b) {
                continue;
            }

            let moved = b.step(Direction::Down);
            if let Some(c) = grid.get(moved) {
                match c {
                    '.' => {
                        new_beams.push(moved);
                    }
                    '^' => {
                        new_beams.push(Point::new(moved.x - 1, moved.y));
                        new_beams.push(Point::new(moved.x + 1, moved.y));
                    }
                    _ => unreachable!(),
                }
            }
        }

        beams = new_beams;
        split_count = split_count.max(beams.len());
    }

    split_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_part_one_example() {
        let result = part_one(INPUT);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(INPUT);
        assert_eq!(result, 40);
    }
}
