/// Utility functions for Advent of Code 2020.
///
/// These are functions that can likely be used across multiple days. Having them bundled in this
/// crate prevents having to rewrite the same generic code for each day, saving time.
use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

/// Read limes from an input file.
///
/// Reads lines from the input file `file_name`. Each line is parsed as `T`, where `T` imlements
/// `FromStr`.
///
/// Example usage:
///
/// ```
/// use util::read_input;
///
/// // See `int_input` file, which has the numbers 1, 2, 3, and 4 separated by newline characters.
/// let input = read_input::<u8>("int_input");
/// assert_eq!(input.collect::<Vec<u8>>(), vec![1, 2, 3, 4]);
/// ```
pub fn read_input<T>(file_name: &str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(file_name).expect(&format!("Could not open file {}", file_name));
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .map(|line| {
            line.parse::<T>()
                .expect(&format!("Could not parse line {}", line))
        })
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    #[test]
    fn read_input_integer() {
        assert_eq!(
            read_input::<usize>("int_input").collect::<Vec<usize>>(),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    #[should_panic]
    fn no_file() {
        let _ = read_input::<usize>("non_existant_file");
    }

    #[test]
    #[should_panic]
    fn invalid_utf8() {
        let _ = read_input::<String>("invalid_input").collect::<Vec<String>>();
    }

    #[test]
    #[should_panic]
    fn could_not_parse() {
        let _ = read_input::<usize>("float_input").collect::<Vec<usize>>();
    }
}
