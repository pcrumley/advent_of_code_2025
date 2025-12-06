use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString)]
enum Operation {
    #[strum(serialize = "+")]
    Add,
    #[strum(serialize = "*")]
    Mult,
}

#[derive(Debug, Clone)]
struct Problem {
    args: Vec<u64>,
    op: Operation,
}

impl Problem {
    fn new(input: &[String]) -> Self {
        let mut args = vec![];
        for i in 0..input.len() - 1 {
            let v = &input[i];
            args.push(v.parse().unwrap());
        }
        let op = input[input.len() - 1].parse().unwrap();
        Self { args, op }
    }
    fn solve(&self) -> u64 {
        match self.op {
            Operation::Mult => self.args.iter().fold(1, |acc, x| acc * x),
            Operation::Add => self.args.iter().fold(0, |acc, x| acc + x),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Worksheet {
    problems: Vec<Problem>,
}

impl Worksheet {
    fn from_file(fname: impl AsRef<Path>) -> Self {
        let f = File::open(fname.as_ref()).unwrap();
        let reader = BufReader::new(f);

        let mut input_matrix: Vec<Vec<String>> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            // parse all the columns, effectively
            let inputs = line.trim().split_whitespace().map(Into::into).collect();
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

        /*
        let problems = transposed_matrix
            .iter()
            .into_iter()
            .map(|builder| builder.build())
            .collect();
        */
        let problems = vec![];
        Self { problems }
    }
}

fn main() {
    /*
    let pantry = IngredientPantry::from_file("input.txt");
    println!("Part A: `{}`", pantry.num_fresh_available());
    println!("Part B: `{}`", pantry.num_fresh());
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_a() {
        let pantry = IngredientPantry::from_file("test.txt");
        assert_eq!(3, pantry.num_fresh_available())
    }

    #[test]
    fn test_b() {
        let pantry = IngredientPantry::from_file("test.txt");
        assert_eq!(14, pantry.num_fresh())
    }
    */
}
