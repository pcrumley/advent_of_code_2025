use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, EnumString)]
enum Operation {
    #[strum(serialize = "+")]
    Add,
    #[strum(serialize = "*")]
    Mult,
}

enum Part {
    A,
    B,
}

#[derive(Debug, Clone)]
struct Problem {
    args: Vec<String>,
    op: Operation,
}

impl Problem {
    fn new(inputs: &[String]) -> Self {
        let mut args = vec![];
        for i in 0..inputs.len() - 1 {
            args.push(inputs[i].clone());
        }
        let op = inputs[inputs.len() - 1].trim().parse().unwrap();

        Self { args, op }
    }

    fn solve_a(&self) -> u64 {
        match self.op {
            Operation::Mult => self
                .args
                .iter()
                .fold(1, |acc, x| acc * x.trim().parse::<u64>().unwrap()),
            Operation::Add => self
                .args
                .iter()
                .fold(0, |acc, x| acc + x.trim().parse::<u64>().unwrap()),
        }
    }

    fn solve_b(&self) -> u64 {
        // have to take the transpose of all the strings first
        let bytes_arr: Vec<&[u8]> = self.args.iter().map(|s| s.as_bytes()).collect();
        let mut transposed_bytes_arr = vec![vec![b'0'; bytes_arr.len()]; bytes_arr[0].len()];

        for i in 0..bytes_arr.len() {
            for j in 0..bytes_arr[0].len() {
                transposed_bytes_arr[j][i] = bytes_arr[i][j];
            }
        }

        Self {
            args: transposed_bytes_arr
                .into_iter()
                .map(|b| String::from_utf8(b).unwrap())
                .collect(),
            op: self.op,
        }
        .solve_a()
    }
}

#[derive(Debug, Default, Clone)]
struct Worksheet {
    problems: Vec<Problem>,
}

impl Worksheet {
    // this is helper function which just reads the last name of the file
    // and gets the offsets
    fn get_offsets(fname: &Path) -> Vec<usize> {
        let f = File::open(fname).unwrap();
        let reader = BufReader::new(f);

        let last_line = reader.lines().last().unwrap().unwrap();
        // get the offsets by reading the location of the chars that can be read
        // in as a operator in the last line of the file
        // we know its ascii so can be lazy about utf-8
        let mut offsets: Vec<usize> = last_line
            .bytes()
            .enumerate()
            .filter_map(|(i, b)| {
                let as_str = str::from_utf8(std::slice::from_ref(&b)).ok()?;
                let _: Operation = as_str.parse().ok()?;
                Some(i)
            })
            .collect();

        offsets.push(last_line.len() + 1);
        offsets
    }
    fn from_file(fname: impl AsRef<Path>) -> Self {
        let offsets = Self::get_offsets(fname.as_ref());
        let f = File::open(fname.as_ref()).unwrap();
        let reader = BufReader::new(f);

        let mut input_matrix: Vec<Vec<String>> = vec![];

        for line in reader.lines() {
            let line = line.unwrap();
            let line_bytes = line.into_bytes();
            // parse all the columns, effectively
            let inputs = offsets
                .iter()
                .zip(offsets[1..].iter())
                .map(|(&start, &next_start)| {
                    String::from_utf8(line_bytes[start..next_start - 1].to_owned()).unwrap()
                })
                .collect();
            input_matrix.push(inputs);
        }

        // transpose the matrix probably should use ndarray or other linear alg
        // lib
        let mut transposed_matrix =
            vec![vec![String::new(); input_matrix.len()]; input_matrix[0].len()];
        for i in 0..input_matrix.len() {
            for j in 0..input_matrix[0].len() {
                transposed_matrix[j][i] = input_matrix[i][j].to_string();
            }
        }

        let problems = transposed_matrix.iter().map(|m| Problem::new(&m)).collect();
        Self { problems }
    }

    fn sum(&self, part: Part) -> u64 {
        match part {
            Part::A => self.problems.iter().map(|p| p.solve_a()).sum(),
            Part::B => self.problems.iter().map(|p| p.solve_b()).sum(),
        }
    }
}

fn main() {
    let worksheet = Worksheet::from_file("input.txt");
    println!("Part A: `{}`", worksheet.sum(Part::A));
    println!("Part B: `{}`", worksheet.sum(Part::B));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let ws = Worksheet::from_file("test.txt");
        assert_eq!(4277556, ws.sum(Part::A))
    }

    #[test]
    fn test_b() {
        let ws = Worksheet::from_file("test.txt");
        assert_eq!(3263827, ws.sum(Part::B))
    }
}
