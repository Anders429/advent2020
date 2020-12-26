use util::read_input;

fn transform(value: usize, subject_number: usize) -> usize {
    (value * subject_number) % 20201227
}

fn find_loop_size(key: usize) -> usize {
    let mut loop_size = 0;
    let mut value = 1;
    loop {
        loop_size += 1;
        value = transform(value, 7);
        if value == key {
            return loop_size;
        }
    }
}

fn encryption_key(card_public_key: usize, door_public_key: usize) -> usize {
    let card_loop_size = find_loop_size(card_public_key);
    let door_loop_size = find_loop_size(door_public_key);
    let mut result = 1;
    for _ in 0..door_loop_size {
        result = transform(result, card_public_key);
    }
    result
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<usize>(&args[1]).collect::<Vec<usize>>();

    println!("{}", encryption_key(input[0], input[1]));
}
