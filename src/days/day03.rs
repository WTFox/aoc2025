use itertools::Itertools;

fn find_largest(bank: &[i64], start: usize, stop: usize) -> (usize, i64) {
    let (i, v) = &bank[start..stop]
        .iter()
        .enumerate()
        .sorted_by_key(|v| -v.1)
        .take(1)
        .next()
        .unwrap();
    (*i, **v)
}

pub fn part_one(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .map(|bank| {
            let (idx, first_num) = find_largest(&bank, 0, bank.len() - 1);
            let (_, second_num) = find_largest(&bank, idx + 1, bank.len());
            first_num * 10 + second_num
        })
        .sum()
}

fn process_bank(bank: &[i64]) -> i64 {
    let mut new_bank = vec![];
    for i in 11..0 {
        new_bank.push(find_largest(bank, 11 - i, bank.len() - i).1);
    }

    println!("{:?}", new_bank);

    0

    // let mut new_bank = vec![];
    // for i in 0..12 {
    //     new_bank.push(find_largest(&bank, i, bank.len()).1);
    // }
    // new_bank
    //     .iter()
    //     .map(|&x| x.to_string())
    //     .collect::<Vec<_>>()
    //     .concat()
    //     .parse()
    //     .unwrap()
}

pub fn part_two(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .map(|bank| process_bank(&bank))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_one_example() {
        let result = part_one(INPUT);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(INPUT);
        assert_eq!(result, 3121910778619);
    }
}
