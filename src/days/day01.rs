pub fn part_one(input: &str) -> i32 {
    let mut dial = Dial::new(50);
    input
        .lines()
        .map(|line| {
            match line.trim().split_at(1) {
                ("L", by) => dial.rotate_left(by.parse().unwrap()),
                ("R", by) => dial.rotate_right(by.parse().unwrap()),
                _ => -1,
            };
            dial.pos
        })
        .filter(|&line| line == 0)
        .count() as i32
}

pub fn part_two(input: &str) -> i32 {
    let mut dial = Dial::new(50);
    input.lines().fold(0, |count, line| {
        count
            + match line.trim().split_at(1) {
                ("L", by) => dial.rotate_left(by.parse().unwrap()),
                ("R", by) => dial.rotate_right(by.parse().unwrap()),
                _ => 0,
            }
    })
}

struct Dial {
    pos: i32,
}

impl Dial {
    fn new(pos: i32) -> Self {
        Dial { pos }
    }

    fn rotate_left(&mut self, by: i32) -> i32 {
        let mut rotations = 0;
        for _ in 0..by {
            self.pos -= 1;
            if self.pos == -1 {
                self.pos = 99
            }
            if self.pos == 0 {
                rotations += 1;
            }
        }
        rotations
    }

    fn rotate_right(&mut self, by: i32) -> i32 {
        let mut rotations = 0;
        for _ in 0..by {
            self.pos += 1;
            if self.pos == 100 {
                self.pos = 0
            }
            if self.pos == 0 {
                rotations += 1;
            }
        }
        rotations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part_one_example() {
        let result = part_one(INPUT);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(INPUT);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example_step_by_step() {
        let mut dial = Dial::new(50);
        let mut rotations = 0;

        rotations += dial.rotate_left(68);
        assert_eq!(dial.pos, 82);
        assert_eq!(rotations, 1);

        rotations += dial.rotate_left(30);
        assert_eq!(dial.pos, 52);
        assert_eq!(rotations, 1);

        rotations += dial.rotate_right(48);
        assert_eq!(dial.pos, 0);
        assert_eq!(rotations, 2);

        rotations += dial.rotate_left(5);
        assert_eq!(dial.pos, 95);
        assert_eq!(rotations, 2);
    }

    #[test]
    fn test_rotate_left() {
        let mut dial = Dial::new(50);
        dial.rotate_left(3);
        assert_eq!(dial.pos, 47);

        let mut dial = Dial::new(50);
        let count = dial.rotate_left(1000);
        assert_eq!(count, 10);

        let mut dial = Dial::new(50);
        let rotations = dial.rotate_left(68);
        assert_eq!(dial.pos, 82);
        assert_eq!(rotations, 1);

        let mut dial = Dial::new(0);
        dial.rotate_left(12);
        assert_eq!(dial.pos, 88);

        let mut dial = Dial::new(50);
        let mut rotations = dial.rotate_left(68);
        assert_eq!(rotations, 1);
        assert_eq!(dial.pos, 82);
        rotations += dial.rotate_left(30);
        assert_eq!(rotations, 1);
        assert_eq!(dial.pos, 52);

        let mut dial = Dial::new(50);
        let mut rotations = dial.rotate_left(99);
        rotations += dial.rotate_left(99);
        rotations += dial.rotate_left(198);
        assert_eq!(rotations, 4);
        assert_eq!(dial.pos, 54);
    }

    #[test]
    fn test_rotate_right() {
        let mut dial = Dial::new(0);
        dial.rotate_right(3);
        assert_eq!(dial.pos, 3);

        let mut dial = Dial::new(50);
        dial.rotate_right(3);
        assert_eq!(dial.pos, 53);

        let mut dial = Dial::new(50);
        let count = dial.rotate_right(1000);
        assert_eq!(count, 10);

        let mut dial = Dial::new(99);
        dial.rotate_right(1);
        assert_eq!(dial.pos, 0);

        let mut dial = Dial::new(99);
        let rotations = dial.rotate_right(3);
        assert_eq!(rotations, 1);
        assert_eq!(dial.pos, 2);

        let mut dial = Dial::new(99);
        let mut rotations = dial.rotate_right(99);
        rotations += dial.rotate_right(99);
        rotations += dial.rotate_right(99);
        rotations += dial.rotate_right(198);
        assert_eq!(rotations, 5);
        assert_eq!(dial.pos, 94);
    }
}
