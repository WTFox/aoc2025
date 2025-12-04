use pathfinding::matrix::Matrix;

pub fn part_one(input: &str) -> usize {
    let grid = build_grid(input);
    all_tp_locations(&grid)
        .iter()
        .filter(|&&p| tp_neighbor_count_less_than_four(&grid, p))
        .count()
}

pub fn part_two(input: &str) -> usize {
    let mut grid = build_grid(input);
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

fn build_grid(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.trim().lines().map(|l| l.chars())).unwrap()
}

fn remove_tp_at_locations(grid: &mut Matrix<char>, locations: Vec<(usize, usize)>) {
    locations
        .iter()
        .for_each(|&p| *grid.get_mut(p).unwrap() = '.');
}

fn tp_neighbor_count_less_than_four(grid: &Matrix<char>, p: (usize, usize)) -> bool {
    grid.neighbours(p, true)
        .filter(|&p| grid.get(p).unwrap() == &'@')
        .count()
        < 4
}

fn all_tp_locations(grid: &Matrix<char>) -> Vec<(usize, usize)> {
    grid.items()
        .filter(|(_, &v)| v == '@')
        .map(|(p, _)| p)
        .collect::<Vec<_>>()
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
