use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

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
