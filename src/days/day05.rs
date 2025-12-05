pub fn part_one(input: &str) -> usize {
    let (fresh_id_ranges, ingredients) = input.split_once("\n\n").unwrap();
    let merged = build_merged_ranges(fresh_id_ranges);
    ingredients
        .trim()
        .lines()
        .map(|num| num.trim().parse::<i64>().unwrap())
        .filter(|&num| {
            merged
                .iter()
                .any(|&(start, end)| start <= num && num <= end + 1)
        })
        .count()
}

pub fn part_two(input: &str) -> i64 {
    let (fresh_id_ranges, _) = input.split_once("\n\n").unwrap();
    build_merged_ranges(fresh_id_ranges)
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

fn build_merged_ranges(input: &str) -> Vec<(i64, i64)> {
    let fresh_id_ranges = input
        .trim()
        .lines()
        .map(|rng| {
            let (start, end) = rng
                .split_once("-")
                .and_then(|(l, r)| Some((l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap())))
                .expect("could not parse number");
            (start, end)
        })
        .collect::<Vec<_>>();

    merge_ranges(fresh_id_ranges)
}

/// Takes a vector of (start, end) and merges the overlapping ranges. Returns new vector
fn merge_ranges(mut fresh_id_ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    fresh_id_ranges.sort_by_key(|r| r.0);
    let mut merged = vec![fresh_id_ranges[0]];
    for range in fresh_id_ranges.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if range.0 <= last.1 + 1 {
            last.1 = last.1.max(range.1);
        } else {
            merged.push((range.0, range.1));
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_one_example() {
        let result = part_one(INPUT);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(INPUT);
        assert_eq!(result, 14);
    }
}
