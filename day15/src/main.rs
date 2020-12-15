use util::read_input;
use std::collections::HashMap;

fn get_numbers(s: &str) -> Box<[usize]> {
    s.split(',').map(|s| usize::from_str_radix(s, 10).unwrap()).collect::<Vec<_>>().into_boxed_slice()
}

fn find_index(input: &[usize], mut index: usize) -> usize {
    let mut map = HashMap::new();
    let mut current_index = 0;

    for i in input {
        map.insert(*i, current_index);
        current_index += 1;
    }

    let mut last = 0;

    while current_index <= (index - 2) {
        last = match map.insert(last, current_index) {
            Some(prev_index) => current_index - prev_index,
            None => 0,
        };
        current_index += 1;
    }

    last
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = get_numbers(&read_input::<String>(&args[1]).collect::<Vec<String>>()[0]);

    println!("{}", find_index(&input, 2020));
    println!("{}", find_index(&input, 30000000));
}
