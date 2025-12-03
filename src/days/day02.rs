use fancy_regex::Regex;
use rayon::prelude::*;

pub fn part_one(input: &str) -> i64 {
    process(input, false)
}

pub fn part_two(input: &str) -> i64 {
    process(input, true)
}

fn process(input: &str, repeating: bool) -> i64 {
    input
        .trim()
        .split(",")
        .flat_map(|rng| {
            let (start, end) = rng
                .split_once("-")
                .and_then(|(l, r)| Some((l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap())))
                .expect("could not parse number");
            start..=end
        })
        .collect::<Vec<i64>>()
        .par_iter()
        .filter(|num| is_repeating(&format!("{}", num), repeating))
        .sum()
}

fn is_repeating(num: &str, more_than_twice: bool) -> bool {
    match more_than_twice {
        true => Regex::new(r"^(\d+?)\1+$")
            .unwrap()
            .is_match(num)
            .unwrap_or(false),
        false => Regex::new(r"^(\d+)\1$")
            .unwrap()
            .is_match(num)
            .unwrap_or(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_one_example() {
        let result = part_one(INPUT);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(INPUT);
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn test_is_repeating() {
        assert!(is_repeating("1010", true));
        assert!(is_repeating("11", true));
        assert!(is_repeating("111", true));
        assert!(is_repeating("1188511885", true));
        assert!(is_repeating("2121212121", true));
        assert!(is_repeating("22", true));
        assert!(is_repeating("222222", true));
        assert!(is_repeating("38593859", true));
        assert!(is_repeating("446446", true));
        assert!(is_repeating("565656", true));
        assert!(is_repeating("824824824", true));
        assert!(is_repeating("99", true));
    }
}
