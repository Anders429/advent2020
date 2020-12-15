use util::read_input;

fn get_numbers(s: &str) -> Box<[usize]> {
    s.split(',')
        .map(|s| usize::from_str_radix(s, 10).unwrap())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn find_index(input: &[usize], index: usize) -> usize {
    let mut map = vec![None; index];
    let mut current_index = 0;

    for i in input {
        map[*i] = Some(current_index);
        current_index += 1;
    }

    let mut last = 0;

    while current_index <= (index - 2) {
        let new_last = match map[last] {
            Some(prev_index) => current_index - prev_index,
            None => 0,
        };
        map[last] = Some(current_index);
        current_index += 1;
        last = new_last;
    }

    last
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = get_numbers(&read_input::<String>(&args[1]).collect::<Vec<String>>()[0]);

    println!("{}", find_index(&input, 2020));
    println!("{}", find_index(&input, 30000000));
}
