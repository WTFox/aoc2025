#[derive(Debug, Default)]
pub struct Dial {
    pub pos: i32,
}

impl Dial {
    pub fn new(pos: i32) -> Self {
        Dial { pos }
    }

    pub fn rotate_left(&mut self, amount: i32) -> i32 {
        self.pos = (self.pos - amount) % 100;
        if self.pos < 0 {
            self.pos += 100
        }
        self.pos
    }

    pub fn rotate_right(&mut self, amount: i32) -> i32 {
        self.pos = (self.pos + amount) % 100;
        if self.pos < 0 {
            self.pos += 100
        }
        self.pos
    }
}

pub fn part_one(input: &str) -> i32 {
    let mut dial = Dial::new(50);
    let mut count = 0;

    input.trim().lines().for_each(|line| {
        if line.is_empty() {
            return;
        }

        let line = line.trim().to_ascii_lowercase();
        match line.split_at(1) {
            ("r", by) => {
                println!("rotating right by {}", by);
                dial.rotate_right(by.parse().unwrap());
                if dial.pos == 0 {
                    count += 1;
                }
            }
            ("l", by) => {
                println!("rotating left by {}", by);
                dial.rotate_left(by.parse().unwrap());
                if dial.pos == 0 {
                    count += 1;
                }
            }
            _ => panic!("invalid instruction"),
        }
    });

    count
}

pub fn part_two(_input: &str) -> i32 {
    42
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
    fn test_part_one_example_one() {
        let result = part_one(INPUT);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_rotate_left() {
        let mut dial = Dial::new(50);
        dial.rotate_left(3);
        assert_eq!(dial.pos, 47);

        let mut dial = Dial::new(50);
        dial.rotate_left(68);
        assert_eq!(dial.pos, 82);

        let mut dial = Dial::new(0);
        dial.rotate_left(12);
        assert_eq!(dial.pos, 88);

        let mut dial = Dial::new(50);
        dial.rotate_left(68);
        dial.rotate_left(30);
        assert_eq!(dial.pos, 52);
    }

    #[test]
    fn test_rotate_right() {
        let mut dial = Dial::new(0);
        dial.rotate_right(3);
        assert_eq!(dial.pos, 3);

        let mut dial = Dial::new(50);
        dial.rotate_right(3);
        assert_eq!(dial.pos, 53);

        let mut dial = Dial::new(99);
        dial.rotate_right(1);
        assert_eq!(dial.pos, 0);

        let mut dial = Dial::new(99);
        dial.rotate_right(3);
        assert_eq!(dial.pos, 2);
    }
}
