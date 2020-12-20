use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use util::read_input;

#[derive(Copy, Clone, Debug)]
enum Cube {
    Active,
    Inactive,
}

type Grid = HashMap<(isize, isize, isize), Cube>;

fn parse_input(input: &[String]) -> Grid {
    let mut result = HashMap::new();

    for (i, s) in input.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            match c {
                '#' => {
                    result.insert((j as isize, i as isize, 0), Cube::Active);
                }
                _ => {}
            }
        }
    }

    result
}

type FourDimensionGrid = HashMap<(isize, isize, isize, isize), Cube>;

fn add_dimension(grid: &Grid) -> FourDimensionGrid {
    let mut result = FourDimensionGrid::new();

    for (x, y, z) in grid.keys() {
        result.insert((*x, *y, *z, 0), grid[&(*x, *y, *z)]);
    }

    result
}

fn iterate(grid: &Grid, iterations: usize) -> usize {
    let mut mut_grid: Grid = grid.clone();

    for _ in 0..iterations {
        let mut new_grid: Grid = HashMap::new();

        let mut min_x: isize = 0;
        let mut max_x: isize = 0;
        let mut min_y: isize = 0;
        let mut max_y: isize = 0;
        let mut min_z: isize = 0;
        let mut max_z: isize = 0;

        for (x, y, z) in mut_grid.keys() {
            // Count active neighbors.
            let mut active_count = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    for k in -1..=1 {
                        // Skip self.
                        if i == 0 && j == 0 && k == 0 {
                            continue;
                        }
                        if let Cube::Active = mut_grid
                            .get(&(x + i, y + j, z + k))
                            .unwrap_or(&Cube::Inactive)
                        {
                            active_count += 1;
                        }
                    }
                }
            }
            match mut_grid[&(*x, *y, *z)] {
                Cube::Active => {
                    if active_count >= 2 && active_count <= 3 {
                        println!(
                            "Inserting active at {} {} {} with count {}",
                            x, y, z, active_count
                        );
                        new_grid.insert((*x, *y, *z), Cube::Active);
                    }
                }
                Cube::Inactive => {
                    if active_count == 3 {
                        println!(
                            "Inserting active at {} {} {} with count {}",
                            x, y, z, active_count
                        );
                        new_grid.insert((*x, *y, *z), Cube::Active);
                    }
                }
            }

            min_x = min(min_x, *x);
            min_y = min(min_y, *y);
            min_z = min(min_z, *z);
            max_x = max(max_x, *x);
            max_y = max(max_y, *y);
            max_z = max(max_z, *z);
        }

        // Also do the inactives.
        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    match mut_grid.get(&(x, y, z)).unwrap_or(&Cube::Inactive) {
                        Cube::Active => {}
                        Cube::Inactive => {
                            // Count active neighbors.
                            let mut active_count = 0;
                            for i in -1..=1 {
                                for j in -1..=1 {
                                    for k in -1..=1 {
                                        // Skip self.
                                        if i == 0 && j == 0 && k == 0 {
                                            continue;
                                        }
                                        if let Cube::Active = mut_grid
                                            .get(&(x + i, y + j, z + k))
                                            .unwrap_or(&Cube::Inactive)
                                        {
                                            active_count += 1;
                                        }
                                    }
                                }
                            }
                            if active_count == 3 {
                                println!(
                                    "Inserting active at {} {} {} with count {}",
                                    x, y, z, active_count
                                );
                                new_grid.insert((x, y, z), Cube::Active);
                            }
                        }
                    }
                }
            }
        }

        mut_grid = new_grid;
    }

    mut_grid
        .values()
        .filter(|v| matches!(v, Cube::Active))
        .count()
}

fn iterate_4d(grid: &FourDimensionGrid, iterations: usize) -> usize {
    let mut mut_grid: FourDimensionGrid = grid.clone();

    dbg!(&mut_grid);

    for _ in 0..iterations {
        let mut new_grid: FourDimensionGrid = HashMap::new();

        let mut min_x: isize = 0;
        let mut max_x: isize = 0;
        let mut min_y: isize = 0;
        let mut max_y: isize = 0;
        let mut min_z: isize = 0;
        let mut max_z: isize = 0;
        let mut min_w: isize = 0;
        let mut max_w: isize = 0;

        for (x, y, z, w) in mut_grid.keys() {
            // Count active neighbors.
            let mut active_count = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    for k in -1..=1 {
                        for l in -1..=1 {
                            // Skip self.
                            if i == 0 && j == 0 && k == 0 && l == 0 {
                                continue;
                            }
                            if let Cube::Active = mut_grid
                                .get(&(x + i, y + j, z + k, w + l))
                                .unwrap_or(&Cube::Inactive)
                            {
                                active_count += 1;
                            }
                        }
                    }
                }
            }
            match mut_grid[&(*x, *y, *z, *w)] {
                Cube::Active => {
                    if active_count >= 2 && active_count <= 3 {
                        //println!("Inserting active at {} {} {} with count {}", x, y, z, active_count);
                        new_grid.insert((*x, *y, *z, *w), Cube::Active);
                    }
                }
                Cube::Inactive => {
                    if active_count == 3 {
                        //println!("Inserting active at {} {} {} with count {}", x, y, z, active_count);
                        new_grid.insert((*x, *y, *z, *w), Cube::Active);
                    }
                }
            }

            min_x = min(min_x, *x);
            min_y = min(min_y, *y);
            min_z = min(min_z, *z);
            min_w = min(min_w, *w);
            max_x = max(max_x, *x);
            max_y = max(max_y, *y);
            max_z = max(max_z, *z);
            max_w = max(max_w, *w);
        }

        dbg!(min_x);
        dbg!(min_y);
        dbg!(min_z);
        dbg!(min_w);
        dbg!(max_x);
        dbg!(max_y);
        dbg!(max_z);
        dbg!(max_w);

        // Also do the inactives.
        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    for w in (min_w - 1)..=(max_w + 1) {
                        match mut_grid.get(&(x, y, z, w)).unwrap_or(&Cube::Inactive) {
                            Cube::Active => {}
                            Cube::Inactive => {
                                // Count active neighbors.
                                let mut active_count = 0;
                                for i in -1..=1 {
                                    for j in -1..=1 {
                                        for k in -1..=1 {
                                            for l in -1..=1 {
                                                // Skip self.
                                                if i == 0 && j == 0 && k == 0 && l == 0 {
                                                    continue;
                                                }
                                                if let Cube::Active = mut_grid
                                                    .get(&(x + i, y + j, z + k, w + l))
                                                    .unwrap_or(&Cube::Inactive)
                                                {
                                                    active_count += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                                if active_count == 3 {
                                    //println!("Inserting active at {} {} {} with count {}", x, y, z, active_count);
                                    new_grid.insert((x, y, z, w), Cube::Active);
                                }
                            }
                        }
                    }
                }
            }
        }

        mut_grid = new_grid;

        dbg!(&mut_grid);
    }

    mut_grid
        .values()
        .filter(|v| matches!(v, Cube::Active))
        .count()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();

    let grid = parse_input(&input);

    println!("{}", iterate(&grid, 6));

    let grid = add_dimension(&grid);

    println!("{}", iterate_4d(&grid, 6));
}
