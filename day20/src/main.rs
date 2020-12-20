use num_integer::Roots;
use std::collections::{HashSet, VecDeque};
use util::read_input;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pixel {
    Dark,
    Light,
    Monster,
}

#[derive(Clone, Debug)]
struct Tile {
    id: usize,
    grid: Box<[Box<[Pixel]>]>,
}

impl Tile {
    fn rotate_clockwise(&self) -> Self {
        let mut rotated_grid = vec![Vec::with_capacity(self.grid.len()); self.grid.len()];

        for row in self.grid.iter() {
            for (j, pixel) in row.iter().enumerate() {
                rotated_grid[j].insert(0, *pixel);
            }
        }

        Self {
            id: self.id,
            grid: rotated_grid
                .iter()
                .map(|row| row.clone().into_boxed_slice())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn rotate_counterclockwise(&self) -> Self {
        let mut rotated_grid = vec![Vec::with_capacity(self.grid.len()); self.grid.len()];

        for row in self.grid.iter() {
            for (j, pixel) in row.iter().enumerate() {
                rotated_grid[row.len() - 1 - j].push(*pixel);
            }
        }

        Self {
            id: self.id,
            grid: rotated_grid
                .iter()
                .map(|row| row.clone().into_boxed_slice())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn rotate_180(&self) -> Self {
        let mut rotated_grid = vec![Vec::with_capacity(self.grid.len()); self.grid.len()];

        for (i, row) in self.grid.iter().enumerate() {
            for pixel in row.iter() {
                rotated_grid[row.len() - 1 - i].insert(0, *pixel);
            }
        }

        Self {
            id: self.id,
            grid: rotated_grid
                .iter()
                .map(|row| row.clone().into_boxed_slice())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    // It doesn't actually matter if we do it on x-axis or y-axis.
    fn reflect(&self) -> Self {
        let mut rotated_grid = vec![Vec::with_capacity(self.grid.len()); self.grid.len()];

        for (i, row) in self.grid.iter().enumerate() {
            for pixel in row.iter() {
                rotated_grid[i].insert(0, *pixel);
            }
        }

        Self {
            id: self.id,
            grid: rotated_grid
                .iter()
                .map(|row| row.clone().into_boxed_slice())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn top(&self) -> Box<[Pixel]> {
        self.grid.first().unwrap().clone()
    }

    fn bottom(&self) -> Box<[Pixel]> {
        self.grid.last().unwrap().clone()
    }

    fn right(&self) -> Box<[Pixel]> {
        self.grid
            .iter()
            .map(|row| row.last().unwrap())
            .cloned()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn left(&self) -> Box<[Pixel]> {
        self.grid
            .iter()
            .map(|row| row.first().unwrap())
            .cloned()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn remove_borders(&self) -> Box<[Box<[Pixel]>]> {
        self.grid[1..self.grid.len() - 1]
            .iter()
            .map(|row| row[1..row.len() - 1].to_vec().into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

fn parse_input(input: &[String]) -> Box<[Tile]> {
    let mut result = Vec::new();

    let mut id = 0;
    let mut grid = Vec::new();

    for line in input {
        if line.is_empty() {
            result.push(Tile {
                id,
                grid: grid.into_boxed_slice(),
            });
            id = 0;
            grid = Vec::new();
            continue;
        }
        if line.starts_with("Tile ") {
            id = usize::from_str_radix(
                &line.chars().filter(|c| c.is_digit(10)).collect::<String>(),
                10,
            )
            .unwrap();
        } else {
            grid.push(
                line.chars()
                    .map(|c| match c {
                        '.' => Pixel::Dark,
                        '#' => Pixel::Light,
                        _ => {
                            unreachable!()
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }
    }

    result.into_boxed_slice()
}

fn arrange(tiles: &[Tile]) -> usize {
    let len = tiles.len().sqrt();
    dbg!(len);
    // BFS.
    let mut queue: VecDeque<(Vec<Vec<Tile>>, HashSet<usize>)> = VecDeque::new();
    queue.push_back((Vec::new(), HashSet::new()));

    while !queue.is_empty() {
        let (picture, used_tiles) = queue.pop_front().unwrap();

        if picture.len() == len && picture.last().unwrap().len() == len {
            return picture.first().unwrap().first().unwrap().id
                * picture.first().unwrap().last().unwrap().id
                * picture.last().unwrap().first().unwrap().id
                * picture.last().unwrap().last().unwrap().id;
        }

        for tile in tiles {
            if used_tiles.contains(&tile.id) {
                continue;
            }
            for new_tile in &[
                tile.clone(),
                tile.rotate_clockwise(),
                tile.rotate_counterclockwise(),
                tile.rotate_180(),
                tile.reflect(),
                tile.reflect().rotate_clockwise(),
                tile.reflect().rotate_counterclockwise(),
                tile.reflect().rotate_180(),
            ] {
                let mut new_picture = picture.clone();
                if new_picture.is_empty() {
                    new_picture.push(vec![new_tile.clone()]);
                } else if new_picture.last().unwrap().len() == len {
                    if new_picture.last().unwrap().first().unwrap().bottom() == new_tile.top() {
                        new_picture.push(vec![new_tile.clone()]);
                    } else {
                        continue;
                    }
                } else {
                    if new_picture.len() == 1
                        && new_picture.last().unwrap().last().unwrap().right() == new_tile.left()
                    {
                        new_picture.last_mut().unwrap().push(new_tile.clone());
                    } else if new_picture.last().unwrap().last().unwrap().right() == new_tile.left()
                        && new_picture[new_picture.len() - 2][new_picture.last().unwrap().len()]
                            .bottom()
                            == new_tile.top()
                    {
                        new_picture.last_mut().unwrap().push(new_tile.clone());
                    } else {
                        continue;
                    }
                }

                let mut new_used_tiles = used_tiles.clone();
                new_used_tiles.insert(new_tile.id);
                queue.push_front((new_picture, new_used_tiles));
            }
        }
    }

    0
}

fn arrange_and_get_ids(tiles: &[Tile]) -> Box<[Box<[Tile]>]> {
    let len = tiles.len().sqrt();
    dbg!(len);
    // BFS.
    let mut queue: VecDeque<(Vec<Vec<Tile>>, HashSet<usize>)> = VecDeque::new();
    queue.push_back((Vec::new(), HashSet::new()));

    while !queue.is_empty() {
        let (picture, used_tiles) = queue.pop_front().unwrap();

        if picture.len() == len && picture.last().unwrap().len() == len {
            return picture
                .iter()
                .map(|v| v.clone().into_boxed_slice())
                .collect::<Vec<_>>()
                .into_boxed_slice();
        }

        for tile in tiles {
            if used_tiles.contains(&tile.id) {
                continue;
            }
            for new_tile in &[
                tile.clone(),
                tile.rotate_clockwise(),
                tile.rotate_counterclockwise(),
                tile.rotate_180(),
                tile.reflect(),
                tile.reflect().rotate_clockwise(),
                tile.reflect().rotate_counterclockwise(),
                tile.reflect().rotate_180(),
            ] {
                let mut new_picture = picture.clone();
                if new_picture.is_empty() {
                    new_picture.push(vec![new_tile.clone()]);
                } else if new_picture.last().unwrap().len() == len {
                    if new_picture.last().unwrap().first().unwrap().bottom() == new_tile.top() {
                        new_picture.push(vec![new_tile.clone()]);
                    } else {
                        continue;
                    }
                } else {
                    if new_picture.len() == 1
                        && new_picture.last().unwrap().last().unwrap().right() == new_tile.left()
                    {
                        new_picture.last_mut().unwrap().push(new_tile.clone());
                    } else if new_picture.last().unwrap().last().unwrap().right() == new_tile.left()
                        && new_picture[new_picture.len() - 2][new_picture.last().unwrap().len()]
                            .bottom()
                            == new_tile.top()
                    {
                        new_picture.last_mut().unwrap().push(new_tile.clone());
                    } else {
                        continue;
                    }
                }

                let mut new_used_tiles = used_tiles.clone();
                new_used_tiles.insert(new_tile.id);
                queue.push_front((new_picture, new_used_tiles));
            }
        }
    }

    Vec::new().into_boxed_slice()
}

fn parse_output(output: &[String]) -> Box<[Tile]> {
    let mut result = Vec::new();

    let mut id = 0;
    let mut grid = Vec::new();
    let mut row = Vec::new();

    for line in output {
        if line.starts_with("            id: ") {
            id = usize::from_str_radix(
                &line.chars().filter(|c| c.is_digit(10)).collect::<String>(),
                10,
            )
            .unwrap();
        } else if line.ends_with("Dark,") {
            row.push(Pixel::Dark);
        } else if line.ends_with("Light,") {
            row.push(Pixel::Light);
        } else if !row.is_empty() {
            grid.push(row.into_boxed_slice());
            row = Vec::new();
        }

        if line == "        }," {
            result.push(Tile {
                id,
                grid: grid.into_boxed_slice(),
            });
            id = 0;
            grid = Vec::new();
        }
    }
    result.into_boxed_slice()
}

fn make_full_picture(tiles: &[Tile]) -> Box<[Box<[Pixel]>]> {
    let len = tiles.len().sqrt();

    let height = tiles[0].grid.len() - 2;

    let mut result = Vec::new();

    for (i, tile) in tiles.iter().enumerate() {
        if i % len == 0 {
            for _ in 0..height {
                result.push(Vec::new());
            }
        }
        let grid = tile.remove_borders();
        for j in 0..height {
            let res_len = result.len();
            result[res_len + j - height].extend(grid[j].iter());
        }
    }
    result
        .iter()
        .map(|v| v.clone().into_boxed_slice())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn find_monsters(picture: Box<[Box<[Pixel]>]>) -> usize {
    let mut picture_clone = picture.clone();

    let top = "                  # ";
    let mid = "#    ##    ##    ###";
    let bot = " #  #  #  #  #  #   ";

    for i in 0..(picture.len() - 3) {
        'pixel: for j in 0..(picture[0].len() - 20) {
            for k in 0..3 {
                for l in 0..20 {
                    let pixel = picture[i + k][j + l];
                    let c = match k {
                        0 => top.chars().nth(l).unwrap(),
                        1 => mid.chars().nth(l).unwrap(),
                        2 => bot.chars().nth(l).unwrap(),
                        _ => unreachable!(),
                    };
                    //dbg!(&c);
                    match c {
                        '#' => {
                            if !matches!(pixel, Pixel::Light) {
                                continue 'pixel;
                            }
                        }
                        _ => {}
                    }

                    if k == 2 && l == 19 {
                        println!("Found monster");
                        for m in 0..3 {
                            for n in 0..20 {
                                let c = match m {
                                    0 => top.chars().nth(n).unwrap(),
                                    1 => mid.chars().nth(n).unwrap(),
                                    2 => bot.chars().nth(n).unwrap(),
                                    _ => unreachable!(),
                                };
                                match c {
                                    '#' => {
                                        //dbg!(&[i+m, j+n]);
                                        picture_clone[i + m][j + n] = Pixel::Monster;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    picture_clone
        .iter()
        .map(|r| r.iter().filter(|p| matches!(p, Pixel::Light)).count())
        .sum()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();

    // Part 1
    // let tiles = parse_input(&input);
    // println!("{}", arrange(&tiles));

    // dbg!(arrange_and_get_ids(&tiles));

    let tiles = parse_output(&input);
    let picture = make_full_picture(&tiles);
    // ROtate the picture and flip it.
    let full_tile = Tile {
        id: 0,
        grid: picture.clone(),
    };
    println!("{}", find_monsters(picture));
    println!("{}", find_monsters(full_tile.rotate_clockwise().grid));
    println!(
        "{}",
        find_monsters(full_tile.rotate_counterclockwise().grid)
    );
    println!("{}", find_monsters(full_tile.rotate_180().grid));
    println!("{}", find_monsters(full_tile.reflect().grid));
    println!(
        "{}",
        find_monsters(full_tile.reflect().rotate_clockwise().grid)
    );
    println!(
        "{}",
        find_monsters(full_tile.reflect().rotate_counterclockwise().grid)
    );
    println!("{}", find_monsters(full_tile.reflect().rotate_180().grid));
}
