use util::read_input;

#[derive(Clone, Copy, Debug)]
enum State {
    Empty,
    Occupied,
    Floor,
}

fn make_grid(input: &[String]) -> Vec<Vec<State>> {
    input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => State::Floor,
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    _ => State::Floor,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn iterate_until_change(grid: &Vec<Vec<State>>) -> usize {
    let mut mut_grid = grid.clone();

    let mut changed = true;

    while changed {
        changed = false;

        let mut_grid_clone = mut_grid.clone();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match mut_grid[i][j] {
                    State::Floor => {
                        continue;
                    }
                    _ => {}
                }
                let mut count = 0;
                for k in -1..=1 {
                    for l in -1..=1 {
                        if k == 0 && l == 0 {
                            continue;
                        }
                        let i_prime = (i as isize) + k;
                        let j_prime = (j as isize) + l;
                        if i_prime < 0
                            || i_prime >= grid.len() as isize
                            || j_prime < 0
                            || j_prime >= grid[0].len() as isize
                        {
                            continue;
                        }
                        match mut_grid_clone[i_prime as usize][j_prime as usize] {
                            State::Occupied => count += 1,
                            _ => {}
                        }
                    }
                }
                match mut_grid_clone[i][j] {
                    State::Empty => {
                        if count == 0 {
                            mut_grid[i][j] = State::Occupied;
                            changed = true;
                        }
                    }
                    State::Occupied => {
                        if count >= 4 {
                            mut_grid[i][j] = State::Empty;
                            changed = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    mut_grid
        .iter()
        .map(|v| {
            v.iter()
                .filter(|s| match s {
                    State::Occupied => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn iterate_until_change_2(grid: &Vec<Vec<State>>) -> usize {
    let mut mut_grid = grid.clone();

    let mut changed = true;

    while changed {
        changed = false;

        let mut_grid_clone = mut_grid.clone();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match mut_grid[i][j] {
                    State::Floor => {
                        continue;
                    }
                    _ => {}
                }
                let mut count = 0;

                for k in -1..=1 {
                    for l in -1..=1 {
                        if k == 0 && l == 0 {
                            continue;
                        }
                        for m in 1..std::cmp::max(grid.len(), grid[0].len()) {
                            let i_prime = (i as isize) + (k * m as isize);
                            let j_prime = (j as isize) + (l * m as isize);
                            if i_prime < 0
                                || i_prime >= grid.len() as isize
                                || j_prime < 0
                                || j_prime >= grid[0].len() as isize
                            {
                                continue;
                            }
                            match mut_grid_clone[i_prime as usize][j_prime as usize] {
                                State::Occupied => {
                                    count += 1;
                                    break;
                                }
                                State::Empty => {
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                match mut_grid_clone[i][j] {
                    State::Empty => {
                        if count == 0 {
                            mut_grid[i][j] = State::Occupied;
                            changed = true;
                        }
                    }
                    State::Occupied => {
                        if count >= 5 {
                            mut_grid[i][j] = State::Empty;
                            changed = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    mut_grid
        .iter()
        .map(|v| {
            v.iter()
                .filter(|s| match s {
                    State::Occupied => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();

    let grid = make_grid(&input);

    println!("{}", iterate_until_change(&grid));
    println!("{}", iterate_until_change_2(&grid));
}
