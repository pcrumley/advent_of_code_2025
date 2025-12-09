use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
enum TileType {
    Empty,
    Splitter { been_hit: bool },
    BeamSource,
    Beam,
}

impl From<char> for TileType {
    fn from(c: char) -> Self {
        match c {
            '.' => TileType::Empty,
            '^' => TileType::Splitter { been_hit: false },
            'S' => TileType::BeamSource,
            '|' => TileType::Beam,
            _o => panic!("unexpected input"),
        }
    }
}

impl From<TileType> for char {
    fn from(tile: TileType) -> Self {
        match tile {
            TileType::Empty => '.',
            TileType::Splitter { .. } => '^',
            TileType::BeamSource => 'S',
            TileType::Beam => '|',
        }
    }
}

#[derive(Debug, Clone)]
struct Beam {
    x: usize,
    y: usize,
    is_active: bool,
    num_of_paths: usize,
}

#[derive(Debug, Default, Clone)]
struct Factory {
    floor: Vec<Vec<TileType>>,
    beams: Vec<Beam>,
}

impl Factory {
    fn from_file(fname: impl AsRef<Path>) -> Self {
        let f = File::open(fname.as_ref()).unwrap();
        let reader = BufReader::new(f);

        let mut floor: Vec<Vec<TileType>> = vec![];

        for line in reader.lines() {
            let line = line.unwrap();
            floor.push(line.chars().map(TileType::from).collect())
        }

        let mut beams = vec![];
        for y in 0..floor.len() {
            for x in 0..floor[0].len() {
                match floor[y][x] {
                    TileType::BeamSource => beams.push(Beam {
                        x,
                        y,
                        is_active: true,
                        num_of_paths: 1,
                    }),
                    _ => continue,
                }
            }
        }

        Self { floor, beams }
    }

    fn tick(&mut self) {
        let mut next_beams = vec![];
        while let Some(Beam {
            x,
            y,
            is_active,
            num_of_paths,
        }) = self.beams.pop()
        {
            let Some(next_pos) = self.floor.get_mut(y + 1).and_then(|row| row.get_mut(x)) else {
                next_beams.push(Beam {
                    x,
                    y,
                    is_active: false,
                    num_of_paths,
                });
                continue;
            };
            match next_pos {
                TileType::Splitter { .. } => {
                    // handle some issues with undeflow... ahtough i think it actually will be fine
                    if x == 0 {
                        next_beams.push(Beam {
                            x,
                            y,
                            is_active: false,
                            num_of_paths,
                        });
                    } else {
                        self.beams.push(Beam {
                            x: x - 1,
                            y,
                            is_active,
                            num_of_paths,
                        });
                    }

                    self.beams.push(Beam {
                        x: x + 1,
                        y,
                        is_active,
                        num_of_paths,
                    });
                    *next_pos = TileType::Splitter { been_hit: true }
                }
                TileType::Beam => {
                    let beam_to_merge = next_beams
                        .iter_mut()
                        .find(|b| b.x == x && b.y == y + 1)
                        .unwrap();

                    beam_to_merge.num_of_paths += num_of_paths;
                }
                TileType::Empty => {
                    *next_pos = TileType::Beam;
                    next_beams.push(Beam {
                        x,
                        y: y + 1,
                        is_active,
                        num_of_paths,
                    })
                }
                TileType::BeamSource => unreachable!("Should only have 1 beam source"),
            }
        }

        self.beams = next_beams;
    }

    // should probably takea  writer here but too lazy
    fn get_map(&self) -> String {
        let mut map = String::new();
        for row in self.floor.iter() {
            for tile in row {
                map.push((*tile).into())
            }
            map.push('\n');
        }
        map
    }

    fn simulate(mut self, debug: bool) -> Self {
        let mut i = 0;
        while self.beams.iter().any(|b| b.is_active) {
            if debug {
                println!("step {i}");
                println!("num of paths {}", self.beams.len());
                println!("{}", self.get_map());
            }
            self.tick();
            i += 1;
        }

        self
    }

    fn part_a(&self) -> usize {
        self.floor
            .iter()
            .flat_map(|row| row.iter())
            .filter(|t| matches!(t, TileType::Splitter { been_hit: true }))
            .count()
    }

    fn part_b(&self) -> usize {
        self.beams.iter().map(|b| b.num_of_paths).sum()
    }
}

fn main() {
    let factory = Factory::from_file("input.txt").simulate(false);

    println!("Part A: `{}`", factory.part_a());
    println!("Part B: `{}`", factory.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let ws = Factory::from_file("test.txt").simulate(true);
        assert_eq!(21, ws.part_a())
    }

    #[test]
    fn test_b() {
        let ws = Factory::from_file("test.txt").simulate(true);
        assert_eq!(40, ws.part_b())
    }
}
