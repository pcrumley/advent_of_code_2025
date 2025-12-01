/// a struct which holds the the state of the lock
pub struct LockDial {
    loc: usize,
    ended_at_zero: usize,
    clicked_zero: usize,
}

pub enum Direction {
    Right,
    Left,
}

impl LockDial {
    pub fn new() -> Self {
        Self {
            loc: 50,
            ended_at_zero: 0,
            clicked_zero: 0,
        }
    }

    fn rot_right(&mut self) {
        if self.loc == 99 {
            self.loc = 0;
        } else {
            self.loc += 1;
        }
        if self.loc == 0 {
            self.clicked_zero += 1;
        }
    }

    fn rot_left(&mut self) {
        if self.loc == 0 {
            self.loc = 99
        } else {
            self.loc -= 1;
        }
        if self.loc == 0 {
            self.clicked_zero += 1;
        }
    }

    pub fn rot(&mut self, n: usize, direction: Direction) {
        match direction {
            Direction::Right => {
                for _ in 0..n {
                    self.rot_right()
                }
            }
            Direction::Left => {
                for _ in 0..n {
                    self.rot_left()
                }
            }
        }
    }

    pub fn read_instruction(&mut self, instruction: &str) {
        let mut chars = instruction.chars();
        let direction = match chars.next() {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _other => panic!("Unexpected Input"),
        };
        let num_steps = chars.collect::<String>().parse().expect("Must be usize");
        self.rot(num_steps, direction);

        if self.loc == 0 {
            self.ended_at_zero += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_input = vec![
            ("L68", 82),
            ("L30", 52),
            ("R48", 0),
            ("L5", 95),
            ("R60", 55),
            ("L55", 0),
            ("L1", 99),
            ("L99", 0),
            ("R14", 14),
            ("L82", 32),
        ];
        let mut lock = LockDial::new();
        assert_eq!(50, lock.loc);

        for (instruction, outcome) in &test_input {
            lock.read_instruction(instruction);
            assert_eq!(lock.loc, *outcome);
        }

        assert_eq!(lock.ended_at_zero, 3);
        assert_eq!(lock.clicked_zero, 6);
    }
}

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut lock = LockDial::new();
    for line in reader.lines() {
        let line = line.unwrap();
        lock.read_instruction(&line);
    }
    println!("pw is {}", lock.ended_at_zero);
    println!("2nd pw is {}", lock.clicked_zero);
}
