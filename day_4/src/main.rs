use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum TileType {
    Empty,
    PaperRoll,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseTileTypeError;

impl FromStr for TileType {
    type Err = ParseTileTypeError;

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "@" => Ok(TileType::PaperRoll),
            "." => Ok(TileType::Empty),
            _other => Err(ParseTileTypeError),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tile {
    row: usize,
    col: usize,
    _type: TileType,
}

impl Tile {
    fn is_empty(&self) -> bool {
        matches!(self._type, TileType::Empty)
    }

    fn is_paper(&self) -> bool {
        matches!(self._type, TileType::PaperRoll)
    }
}

#[derive(Clone, Debug)]
struct PrinterFloor {
    map: Vec<Vec<Tile>>,
    num_of_row: usize,
    num_of_col: usize,
}

impl PrinterFloor {
    fn from_file(fname: impl AsRef<Path>) -> Self {
        let f = File::open(fname.as_ref()).unwrap();
        let reader = BufReader::new(f);

        let mut map = vec![];
        for (row, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let row: Vec<Tile> = line
                .trim()
                .chars()
                .enumerate()
                .map(|(col, c)| Tile {
                    row,
                    col,
                    _type: c.to_string().parse().unwrap(),
                })
                .collect();
            map.push(row);
        }
        let num_of_row = map.len();
        let num_of_col = map[0].len();
        Self {
            map,
            num_of_row,
            num_of_col,
        }
    }

    fn neighbors(&self, tile: &Tile) -> Vec<&Tile> {
        let rows_to_visit = [
            tile.row.checked_sub(1),
            Some(tile.row),
            (tile.row + 1 < self.num_of_row).then(|| tile.row + 1),
        ];
        let cols_to_visit = [
            tile.col.checked_sub(1),
            Some(tile.col),
            (tile.col + 1 < self.num_of_col).then(|| tile.col + 1),
        ];
        let mut neighbors = vec![];
        for row in rows_to_visit.into_iter().flatten() {
            for col in cols_to_visit.into_iter().flatten() {
                if (row, col) == (tile.row, tile.col) {
                    continue;
                }
                neighbors.push(&self.map[row][col])
            }
        }
        neighbors
    }

    fn is_available(&self, tile: &Tile) -> bool {
        !tile.is_empty()
            && self
                .neighbors(tile)
                .iter()
                .filter(|x| !x.is_empty())
                .count()
                < 4
    }

    fn tiles(&self) -> impl Iterator<Item = &Tile> {
        self.map.iter().flat_map(|x| x.iter())
    }

    fn num_of_rolls(&self) -> usize {
        self.tiles().filter(|t| t.is_paper()).count()
    }

    fn get_available_rolls(&self) -> usize {
        self.tiles().filter(|loc| self.is_available(loc)).count()
    }

    // returns a new floor with the rolls removed and the number of removed rolls
    fn rm_available_rolls(&self) -> (Self, usize) {
        let prev_rolls = self.num_of_rolls();
        let mut new_floor = self.clone();
        for tile in new_floor.map.iter_mut().flat_map(|row| row.iter_mut()) {
            if self.is_available(&tile) {
                tile._type = TileType::Empty;
            }
        }
        let new_rolls = new_floor.num_of_rolls();

        (new_floor, prev_rolls.checked_sub(new_rolls).unwrap())
    }

    fn max_available_rolls(&self) -> usize {
        let mut total_available = 0;
        let (mut new_floor, mut removed_rolls) = self.rm_available_rolls();
        while removed_rolls > 0 {
            total_available += removed_rolls;
            (new_floor, removed_rolls) = new_floor.rm_available_rolls();
        }
        total_available
    }
}

fn main() {
    let printer_floor = PrinterFloor::from_file("input.txt");
    println!("Part A: `{}`", printer_floor.get_available_rolls());
    println!("Part B: `{}`", printer_floor.max_available_rolls());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let printer_floor = PrinterFloor::from_file("test.txt");
        assert_eq!(13, printer_floor.get_available_rolls())
    }

    #[test]
    fn test_b() {
        let printer_floor = PrinterFloor::from_file("test.txt");
        assert_eq!(43, printer_floor.max_available_rolls())
    }
}
