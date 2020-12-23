use util::read_input;
use std::collections::VecDeque;

fn parse_input(input: &[String]) -> VecDeque<usize> {
    let mut result = VecDeque::new();
    for c in input[0].chars() {
        result.push_back(usize::from_str_radix(&c.to_string(), 10).unwrap());
    }
    result
}

fn rotate(cups: &VecDeque<usize>, moves: usize) -> VecDeque<usize> {
    let mut cups = cups.clone();

    let max_val = *cups.iter().max().unwrap();

    for x in 0..moves {
        println!("{}", x);
        //dbg!(&cups);

        let current_cup = cups.pop_front().unwrap();
        cups.push_back(current_cup);

        let first = cups.pop_front().unwrap();
        let second = cups.pop_front().unwrap();
        let third = cups.pop_front().unwrap();

        let mut target_cup = if current_cup != 1 {current_cup - 1} else {max_val};
        while target_cup == first || target_cup == second || target_cup == third {
            target_cup = if target_cup != 1 {target_cup - 1} else {max_val};
        }

        for (i, cup) in cups.iter().enumerate() {
            if *cup == target_cup {
                cups.insert((i + 1) % max_val, third);
                cups.insert((i + 1) % max_val, second);
                cups.insert((i + 1) % max_val, first);
                break;
            }
        }

        // loop {
        //     let next_cup = cups.pop_front().unwrap();
        //     if next_cup == target_cup {
        //         cups.push_front(third);
        //         cups.push_front(second);
        //         cups.push_front(first);
        //         cups.push_front(next_cup);
        //         break;
        //     }
        //     cups.push_back(next_cup);
        // }

        // Get back to the proper next cup.
        loop {
            let cup = cups.pop_front().unwrap();
            cups.push_back(cup);
            if cup == current_cup {
                break;
            }
        }
    }

    cups
}

fn extend_cups(cups: &VecDeque<usize>) -> VecDeque<usize> {
    let mut cups = cups.clone();
    let max_val = *cups.iter().max().unwrap();

    for i in (max_val+1)..=1000000 {
        cups.push_back(i);
    }

    cups
}

fn print_cups(cups: &VecDeque<usize>) -> String {
    let mut cups = cups.clone();

    loop {
        let cup = cups.pop_front().unwrap();
        if cup == 1 {
            break;
        }
        cups.push_back(cup);
    }

    let mut result = String::new();
    for c in cups {
        result.push_str(&c.to_string());
    }
    result
}

fn find_cups(cups: &VecDeque<usize>) -> usize {
    let mut cups = cups.clone();

    loop {
        let cup = cups.pop_front().unwrap();
        if cup == 1 {
            break;
        }
        cups.push_back(cup);
    }

    cups.pop_front().unwrap() * cups.pop_front().unwrap()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();
    let cups = parse_input(&input);

    println!("{}", print_cups(&rotate(&cups, 100)));
    println!("{}", find_cups(&rotate(&extend_cups(&cups), 10000000)))
}
