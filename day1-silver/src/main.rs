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

fn solve(input: &[usize]) -> usize {
    0
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<usize>(&args[1]).collect::<Vec<usize>>();

    println!("{}", solve(&input));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example() {
        let input = &[1721, 979, 366, 299, 675, 1456];

        assert_eq!(solve(input), 514575)
    }
}
