use std::str::FromStr;
use util::read_input;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Directions {
    directions: Vec<Direction>,
}

impl Directions {
    fn add(&self, direction: Direction) -> Self {
        let mut directions = self.directions.clone();
        directions.push(direction);
        Directions {
            directions,
        }
    }

    fn simplify(&self) -> Directions {
        let mut east_count = 0;
        let mut south_east_count = 0;
        let mut south_west_count = 0;
        let mut west_count = 0;
        let mut north_west_count = 0;
        let mut north_east_count = 0;

        for direction in self.directions.iter() {
            match direction {
                Direction::East => east_count += 1,
                Direction::SouthEast => south_east_count += 1,
                Direction::SouthWest => south_west_count += 1,
                Direction::West => west_count += 1,
                Direction::NorthWest => north_west_count += 1,
                Direction::NorthEast => north_east_count += 1,
            }
        }

        // Cancel out opposites.
        while east_count > 0 && west_count > 0 {
            east_count -= 1;
            west_count -= 1;
        }
        while south_east_count > 0 && north_west_count > 0 {
            south_east_count -= 1;
            north_west_count -= 1;
        }
        while south_west_count > 0 && north_east_count > 0 {
            south_west_count -= 1;
            north_east_count -= 1;
        }

        // Cancel out triangles.
        while east_count > 0 && north_west_count > 0 && south_west_count > 0 {
            east_count -= 1;
            north_west_count -= 1;
            south_west_count -= 1;
        }
        while west_count > 0 && north_east_count > 0 && south_east_count > 0 {
            west_count -= 1;
            north_east_count -= 1;
            south_east_count -= 1;
        }

        // Make direct paths
        while east_count > 0 && north_west_count > 0 {
            east_count -= 1;
            north_west_count -= 1;
            north_east_count += 1;
        }
        while east_count > 0 && south_west_count > 0 {
            east_count -= 1;
            south_west_count -= 1;
            south_east_count += 1;
        }
        while west_count > 0 && north_east_count > 0 {
            west_count -= 1;
            north_east_count -= 1;
            north_west_count += 1;
        }
        while west_count > 0 && south_east_count > 0 {
            west_count -= 1;
            south_east_count -= 1;
            south_west_count += 1;
        }
        while south_west_count > 0 && north_west_count > 0 {
            south_west_count -= 1;
            north_west_count -= 1;
            west_count += 1;
        }
        while south_east_count > 0 && north_east_count > 0 {
            south_east_count -= 1;
            north_east_count -= 1;
            east_count += 1;
        }

        // Canonical form.
        let mut directions = Vec::new();
        for _ in 0..east_count {
            directions.push(Direction::East);
        }
        for _ in 0..south_east_count {
            directions.push(Direction::SouthEast);
        }
        for _ in 0..south_west_count {
            directions.push(Direction::SouthWest);
        }
        for _ in 0..west_count {
            directions.push(Direction::West);
        }
        for _ in 0..north_west_count {
            directions.push(Direction::NorthWest);
        }
        for _ in 0..north_east_count {
            directions.push(Direction::NorthEast);
        }

        Self {
            directions,
        }
    }
}

impl FromStr for Directions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let mut result = Vec::new();

        while let Some(c) = chars.next() {
            match c {
                'e' => result.push(Direction::East),
                'w' => result.push(Direction::West),
                's' => match chars.next().unwrap() {
                    'e' => result.push(Direction::SouthEast),
                    'w' => result.push(Direction::SouthWest),
                    _ => unreachable!(),
                },
                'n' => match chars.next().unwrap() {
                    'e' => result.push(Direction::NorthEast),
                    'w' => result.push(Direction::NorthWest),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }

        Ok(Directions {
            directions: result,
        })
    }
}

fn count_black_tiles(input: &[Directions]) -> usize {
    let mut black = HashSet::new();

    for directions in input {
        let simple = directions.simplify();
        // dbg!(&directions);
        // dbg!(&simple);
        if !black.remove(&simple) {
            black.insert(simple);
        }
    }

    black.len()
}

fn flip_tiles(input: &[Directions], iterations: usize) -> usize {
    let mut black = HashSet::new();

    for directions in input {
        let simple = directions.simplify();
        if !black.remove(&simple) {
            black.insert(simple);
        }
    }

    //dbg!(&black);

    for _ in 0..iterations {
        let mut black_tile_count = HashMap::new();
        for directions in black.iter() {
            *black_tile_count.entry(directions.add(Direction::East).simplify()).or_insert(0) += 1;
            *black_tile_count.entry(directions.add(Direction::SouthEast).simplify()).or_insert(0) += 1;
            *black_tile_count.entry(directions.add(Direction::SouthWest).simplify()).or_insert(0) += 1;
            *black_tile_count.entry(directions.add(Direction::West).simplify()).or_insert(0) += 1;
            *black_tile_count.entry(directions.add(Direction::NorthWest).simplify()).or_insert(0) += 1;
            *black_tile_count.entry(directions.add(Direction::NorthEast).simplify()).or_insert(0) += 1;
        }

        //dbg!(&black_tile_count);
        let mut new_black = HashSet::new();

        for tile in black_tile_count.keys() {
            if black.contains(&tile) && *black_tile_count.get(&tile).unwrap() > 0 && *black_tile_count.get(&tile).unwrap() <= 2 {
                new_black.insert(tile.clone());
            } else if !black.contains(&tile) && *black_tile_count.get(&tile).unwrap()  == 2 {
                new_black.insert(tile.clone());
            }
        }

        black = new_black;

        //dbg!(&black);
    }

    

    black.len()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Directions>(&args[1]).collect::<Vec<Directions>>();

    println!("{}", count_black_tiles(&input));
    println!("{}", flip_tiles(&input, 100));
}
