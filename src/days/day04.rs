use crate::{Grid, Point};

pub fn part_one(input: &str) -> usize {
    let grid = Grid::from_str(input);
    all_tp_locations(&grid)
        .iter()
        .filter(|&&p| tp_neighbor_count_less_than_four(&grid, p))
        .count()
}

pub fn part_two(input: &str) -> usize {
    let mut grid = Grid::from_str(input);
    let mut removed = 0;

    loop {
        let candidates = all_tp_locations(&grid)
            .into_iter()
            .filter(|&p| tp_neighbor_count_less_than_four(&grid, p))
            .collect::<Vec<_>>();

        match candidates.len() {
            0 => break,
            _ => removed += candidates.len(),
        }

        remove_tp_at_locations(&mut grid, candidates);
    }

    removed
}

fn remove_tp_at_locations(grid: &mut Grid<char>, locations: Vec<Point>) {
    locations
        .iter()
        .for_each(|&p| *grid.get_mut(p).unwrap() = '.');
}

fn tp_neighbor_count_less_than_four(grid: &Grid<char>, p: Point) -> bool {
    grid.neighbors8(p)
        .into_iter()
        .filter(|&p| grid.get(p).unwrap() == &'@')
        .count()
        < 4
}

fn all_tp_locations(grid: &Grid<char>) -> Vec<Point> {
    grid.iter()
        .filter(|(_, &v)| v == '@')
        .map(|(p, _)| p)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_one_example() {
        let result = part_one(INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(INPUT);
        assert_eq!(result, 43);
    }
}
