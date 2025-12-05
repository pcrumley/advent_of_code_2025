use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Default, Debug, Clone)]
struct IdRanges(Vec<(u64, u64)>);

impl IdRanges {
    fn add(self, (start, finish): (u64, u64)) -> Self {
        let mut output = Self(Vec::with_capacity(self.0.len() + 1));
        let mut had_merge = false;
        for (x1, x2) in self.0.into_iter() {
            if start <= x2 && finish >= x1 {
                had_merge = true;
                output = output.add((x1.min(start), x2.max(finish)));
            } else {
                output.0.push((x1, x2));
            }
        }
        if !had_merge {
            output.0.push((start, finish));
        }

        output
    }

    fn contains(&self, target: &u64) -> bool {
        self.0
            .iter()
            .any(|(start, finish)| start <= target && finish >= target)
    }

    fn total_ids(&self) -> u64 {
        self.0
            .iter()
            .map(|(start, finish)| finish + 1 - start)
            .sum()
    }
}

#[derive(Clone, Debug)]
struct IngredientPantry {
    fresh_ingredients: IdRanges,
    available: Vec<u64>,
}

impl IngredientPantry {
    fn from_file(fname: impl AsRef<Path>) -> Self {
        let f = File::open(fname.as_ref()).unwrap();
        let reader = BufReader::new(f);

        let mut is_preamble = true;
        let mut fresh_ingredients = IdRanges::default();
        let mut available = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            if line.trim().is_empty() {
                is_preamble = false;
                continue;
            }
            if is_preamble {
                let (start, finish) = line.split_once('-').unwrap();
                let start: u64 = start.parse().unwrap();
                let finish: u64 = finish.parse().unwrap();
                fresh_ingredients = fresh_ingredients.add((start, finish));
            } else {
                let id = line.trim().parse().unwrap();
                available.push(id)
            }
        }
        Self {
            fresh_ingredients,
            available,
        }
    }

    fn num_fresh_available(&self) -> usize {
        self.available
            .iter()
            .filter(|&id| self.fresh_ingredients.contains(id))
            .count()
    }

    fn num_fresh(&self) -> u64 {
        self.fresh_ingredients.total_ids()
    }
}

fn main() {
    let pantry = IngredientPantry::from_file("input.txt");
    println!("Part A: `{}`", pantry.num_fresh_available());
    println!("Part B: `{}`", pantry.num_fresh());
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
