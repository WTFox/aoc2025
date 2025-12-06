pub fn part_one(input: &str) -> i64 {
    let grid = input
        .lines()
        .map(|l| l.trim().split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut sum = 0;
    for (i, op) in grid.last().unwrap().iter().enumerate() {
        let col = grid
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != grid.len() - 1)
            .map(|(_, row)| row.into_iter().nth(i).unwrap())
            .map(|&x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let res = match *op {
            "*" => col.iter().fold(1, |acc, x| acc * x),
            "+" => col.iter().fold(0, |acc, x| acc + x),
            _ => todo!(),
        };
        sum += res;
    }

    sum
}

pub fn part_two(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let data_lines = &lines[..lines.len() - 1];
    let op_line = lines.last().unwrap();

    let grid: Vec<Vec<char>> = data_lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let row_count = grid.len();
    let col_count = grid.iter().map(|r| r.len()).max().unwrap_or(0);

    let mut problems: Vec<(char, Vec<usize>)> = vec![];
    let mut current_cols: Vec<usize> = vec![];
    let mut current_op: Option<char> = None;

    for col in (0..col_count).rev() {
        let is_separator =
            (0..row_count).all(|row| grid[row].get(col).unwrap_or(&' ').is_whitespace());

        let op_char = op_line.chars().nth(col).unwrap_or(' ');

        if is_separator {
            if !current_cols.is_empty() {
                if let Some(op) = current_op {
                    problems.push((op, current_cols.clone()));
                }
                current_cols.clear();
                current_op = None;
            }
        } else {
            current_cols.push(col);
            if op_char == '+' || op_char == '*' {
                current_op = Some(op_char);
            }
        }
    }
    if !current_cols.is_empty() {
        if let Some(op) = current_op {
            problems.push((op, current_cols));
        }
    }

    let mut total = 0u64;

    for (op, cols) in problems {
        let nums: Vec<u64> = cols
            .iter()
            .filter_map(|&col| {
                let num_str: String = (0..row_count)
                    .filter_map(|row| grid[row].get(col))
                    .filter(|c| !c.is_whitespace())
                    .collect();

                if num_str.is_empty() {
                    None
                } else {
                    num_str.parse().ok()
                }
            })
            .collect();

        let res: u64 = match op {
            '+' => nums.iter().sum(),
            '*' => nums.iter().product(),
            _ => unreachable!(),
        };
        total += res;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n *   +   *   + ";

    #[test]
    fn test_part_one_example() {
        let result = part_one(INPUT);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(INPUT);
        assert_eq!(result, 3263827);
    }
}
