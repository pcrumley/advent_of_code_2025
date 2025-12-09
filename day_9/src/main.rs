use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Tile {
    x: u64,
    y: u64,
}

#[derive(Eq, PartialEq, Debug)]
struct TileParseError;

impl std::str::FromStr for Tile {
    type Err = TileParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (x, y) = input.split_once(',').ok_or(TileParseError)?;
        let x = x.parse().map_err(|_| TileParseError)?;
        let y = y.parse().map_err(|_| TileParseError)?;

        Ok(Self { x, y })
    }
}

impl Tile {
    fn area(&self, other: &Self) -> u64 {
        // lambda to handle underflow
        let underflow_diff = |x1: u64, x2: u64| -> u64 {
            let abs_diff = x1.max(x2) - x1.min(x2);
            abs_diff + 1
        };

        let mut result = underflow_diff(self.x, other.x);
        result *= underflow_diff(self.y, other.y);
        result
    }

    fn draw_line(&self, other: &Self) -> Vec<Tile> {
        let mut line = vec![];
        let mut x = self.x;
        let mut y = self.y;

        while x != other.x || y != other.y {
            line.push(Tile { x, y });
            x = if x > other.x {
                x - 1
            } else if x < other.x {
                x + 1
            } else {
                x
            };

            y = if y > other.y {
                y - 1
            } else if y < other.y {
                y + 1
            } else {
                y
            };
        }
        line.push(other.clone());

        line
    }
}

#[derive(Debug, Default, Clone)]
struct Factory {
    #[allow(dead_code)]
    red_tiles: Vec<Tile>,
    min_x: u64,
    perimeter: HashSet<Tile>,
}

impl Factory {
    fn build_perimeter(red_tiles: &[Tile]) -> HashSet<Tile> {
        let mut set = red_tiles.iter().zip(red_tiles[1..].iter()).fold(
            HashSet::new(),
            |mut acc, (t1, t2)| {
                acc.extend(t1.draw_line(t2));
                acc
            },
        );

        // the list is wrapping
        set.extend(red_tiles[red_tiles.len() - 1].draw_line(&red_tiles[0]));
        set
    }

    fn print_map(&self) -> String {
        let min_y = self
            .perimeter
            .iter()
            .chain(self.red_tiles.iter())
            .map(|t| t.y)
            .min()
            .unwrap();

        let max_y = self
            .perimeter
            .iter()
            .chain(self.red_tiles.iter())
            .map(|t| t.y)
            .max()
            .unwrap();

        let min_x = self
            .perimeter
            .iter()
            .chain(self.red_tiles.iter())
            .map(|t| t.y)
            .min()
            .unwrap();
        let max_x = self
            .perimeter
            .iter()
            .chain(self.red_tiles.iter())
            .map(|t| t.x)
            .max()
            .unwrap();

        let mut map = String::new();
        for y in min_y - 1..max_y + 2 {
            for x in min_x - 1..max_x + 3 {
                let tile = Tile { x, y };
                if self.red_tiles.contains(&tile) {
                    map.push('#');
                } else if self.inside_perimeter(&tile) {
                    map.push('X');
                } else {
                    map.push('.');
                }
            }
            map.push('\n');
        }
        map
    }

    fn inside_perimeter(&self, tile: &Tile) -> bool {
        if self.perimeter.contains(tile) {
            return true;
        }
        // draw a line from min_x to the tile,
        // if it crosses the perimeter an odd number
        // of times it is inside of the box
        let start = Tile {
            x: self.min_x,
            y: tile.y,
        };
        let path_from_edge: Vec<(bool, Tile)> = start
            .draw_line(tile)
            .into_iter()
            .map(|t| (self.perimeter.contains(&t), t))
            .collect();

        let crossings = path_from_edge
            .windows(2)
            .map(|slice| if !slice[0].0 && slice[1].0 { 1 } else { 0 })
            .sum::<u64>();
        crossings % 2 != 0
    }

    fn from_file(fname: impl AsRef<Path>) -> Self {
        let f = File::open(fname.as_ref()).unwrap();
        let reader = BufReader::new(f);

        let red_tiles: Vec<Tile> = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                line.parse().unwrap()
            })
            .collect();

        let min_x = red_tiles
            .iter()
            .map(|t| t.x)
            .min()
            .unwrap()
            .checked_sub(1)
            .unwrap();

        let perimeter = Self::build_perimeter(&red_tiles);
        Self {
            red_tiles,
            min_x,
            perimeter,
        }
    }

    fn part_a(&self) -> u64 {
        let mut max = 0;
        for (i, t1) in self.red_tiles.iter().enumerate() {
            for t2 in &self.red_tiles[i + 1..] {
                max = max.max(t2.area(t1));
            }
        }
        max
    }

    fn part_b(&self) -> u64 {
        let mut max = 0;
        for (i, t1) in self.red_tiles.iter().enumerate() {
            for t2 in &self.red_tiles[i + 1..] {
                let area = t2.area(t1);
                if area < max {
                    // its small who cares
                    continue;
                }
                let mut perimeter: Vec<_> = t1.draw_line(&Tile { x: t2.x, y: t1.y });
                perimeter.extend_from_slice(&t1.draw_line(&Tile { x: t1.x, y: t2.y }));
                perimeter.extend_from_slice(&t2.draw_line(&Tile { x: t2.x, y: t1.y }));
                perimeter.extend_from_slice(&t2.draw_line(&Tile { x: t1.x, y: t2.y }));
                if perimeter.iter().all(|t| self.inside_perimeter(t)) {
                    max = max.max(area);
                }
            }
        }
        max
    }
}

fn main() {
    let factory = Factory::from_file("input.txt");

    println!("Part A: `{}`", factory.part_a());
    println!("Part B: `{}`", factory.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let ws = Factory::from_file("test.txt");
        assert_eq!(50, ws.part_a())
    }

    #[test]
    fn test_b() {
        let ws = Factory::from_file("test.txt");
        assert_eq!(24, ws.part_b())
    }
}
