use std::io::BufRead;

fn read_input<T>(file_name: &str) -> impl Iterator<Item = T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let file = std::fs::File::open(file_name).unwrap();
    let reader = std::io::BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<T>().unwrap())
}

fn solve(input: &[usize], target: usize) -> Option<(usize, usize)> {
    let mut low_index = 0;
    let mut high_index = input.len() - 1;

    loop {
        let low = input[low_index];
        let high = input[high_index];

        if low + high < target {
            if low_index == input.len() - 1 {
                return None;
            }
            low_index += 1;
        } else if low + high > target {
            if high_index == 0 {
                return None;
            }
            high_index -= 1;
        } else {
            return Some((low, high))
        }
    }
}

fn solve_2(input: &[usize]) -> usize {
    let mut values = input.iter().map(|val| *val).collect::<Vec<usize>>();
    values.sort_unstable();
    let (low, high) = solve(&values, 2020).unwrap();
    low * high
}

fn solve_3(input: &[usize]) -> usize {
    let mut values = input.iter().map(|val| *val).collect::<Vec<usize>>();
    values.sort_unstable();
    for (i, val) in values.iter().enumerate() {
        if let Some((low, high)) = solve(&values[i..], 2020 - val) {
            if low + high + val == 2020 {
                return low * high * val;
            }
        }
    }
    0
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<usize>(&args[1]).collect::<Vec<usize>>();

    println!("{}", solve_2(&input));
    println!("{}", solve_3(&input));
}

#[cfg(test)]
mod tests {
    use crate::{solve_2, solve_3};

    #[test]
    fn example_2() {
        let input = &[1721, 979, 366, 299, 675, 1456];

        assert_eq!(solve_2(input), 514579)
    }

    #[test]
    fn example_3() {
        let input = &[1721, 979, 366, 299, 675, 1456];

        assert_eq!(solve_3(input), 241861950)
    }
}
