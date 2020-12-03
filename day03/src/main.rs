use util::read_input;

fn solve(input: &[String], right: usize, down: usize) -> usize {
    let mut result = 0;

    let mut index = 0;
    let mut iter = input.iter();
    loop {
        let row = match iter.next() {
            Some(r) => r,
            None => {
                break;
            }
        };
        if row.chars().nth(index).unwrap() == '#' {
            result += 1;
        }
        index += right;
        index %= row.len();

        for _ in 0..(down - 1) {
            iter.next();
        }
    }
    result
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();

    println!("{}", solve(&input, 3, 1));
    println!(
        "{}",
        solve(&input, 1, 1)
            * solve(&input, 3, 1)
            * solve(&input, 5, 1)
            * solve(&input, 7, 1)
            * solve(&input, 1, 2)
    );
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_easy() {
        let input = &[
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];

        assert_eq!(solve(input, 3, 1), 7);
    }

    #[test]
    fn test_hard() {
        let input = &[
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];

        assert_eq!(solve(input, 1, 1), 2);
        assert_eq!(solve(input, 3, 1), 7);
        assert_eq!(solve(input, 5, 1), 3);
        assert_eq!(solve(input, 7, 1), 4);
        assert_eq!(solve(input, 1, 2), 2);
        assert_eq!(
            solve(input, 1, 1)
                * solve(input, 3, 1)
                * solve(input, 5, 1)
                * solve(input, 7, 1)
                * solve(input, 1, 2),
            336
        )
    }
}
