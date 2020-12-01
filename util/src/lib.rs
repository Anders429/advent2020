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
/// assert_eq!(input.collect::<Vec<u8>>(), vec!(1, 2, 3, 4));
pub fn read_input<T>(file_name: &str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<T>().unwrap())
}
