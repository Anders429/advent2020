use util::read_input;
use std::collections::HashMap;


fn parse_input(input: &[String]) -> Vec<usize> {
    let mut result = Vec::new();
    for c in input[0].chars() {
        result.push(usize::from_str_radix(&c.to_string(), 10).unwrap());
    }
    result
}

fn make_map(input: &[usize]) -> (HashMap<usize, Vec<usize>>, HashMap<usize, usize>) {
    let mut next = HashMap::new();
    for (i, val) in input.iter().enumerate() {
        let mut nexts = next.entry(*val).or_insert(Vec::new());
        for j in 1..5 {
            nexts.push(input[(i + j) % input.len()])
        }
    }

    let mut prev = HashMap::new();
    for (i, val) in input.iter().enumerate() {
        prev.insert(*val, input[if i == 0 {input.len() - 1} else {i - 1}]);
    }
    
    (next, prev)
}

fn rotate(cups_next: &HashMap<usize, Vec<usize>>, cups_prev: &HashMap<usize, usize>, moves: usize, start_val: usize) -> HashMap<usize, Vec<usize>> {
    let mut cups_next = cups_next.clone();
    let mut cups_prev = cups_prev.clone();

    let max_val = *cups_next.keys().max().unwrap();
    let mut current = start_val;

    for x in 0..moves {
        //println!("{}", x);
        // dbg!(&current);
        // dbg!(&cups_next);
        // dbg!(&cups_prev);

        // Pick up the three cups.
        let mut current_next_four = cups_next.get_mut(&current).unwrap();
        let first = current_next_four.remove(0);
        let second = current_next_four.remove(0);
        let third = current_next_four.remove(0);

        // Find destination cup.
        let mut destination = if current != 1 {current - 1} else {max_val};
        while destination == first || destination == second || destination == third {
            destination = if destination != 1 {destination - 1} else {max_val};
        }
        // dbg!(destination);
        // dbg!(first);
        // dbg!(second);
        // dbg!(third);

        // Close up the circle.
        let current_next = current_next_four[0];
        // dbg!(current_next);
        let current_next_next_four = cups_next.get_mut(&current_next).unwrap();
        //// dbg!(&current_next_next_four);
        let next_first = current_next_next_four[0];
        let next_second = current_next_next_four[1];
        let next_third = current_next_next_four[2];
        // dbg!(next_first);
        // dbg!(next_second);
        // dbg!(next_third);
        let mut current_next_four = cups_next.get_mut(&current).unwrap();
        current_next_four.push(next_first);
        current_next_four.push(next_second);
        current_next_four.push(next_third);
        *cups_prev.get_mut(&current_next).unwrap() = current;

        let current_minus_one = cups_prev[&current];
        // dbg!(current_minus_one);
        let mut current_minus_one_next_four = cups_next.get_mut(&current_minus_one).unwrap();
        current_minus_one_next_four.remove(1);
        current_minus_one_next_four.remove(1);
        current_minus_one_next_four.remove(1);
        current_minus_one_next_four.push(current_next);
        current_minus_one_next_four.push(next_first);
        current_minus_one_next_four.push(next_second);

        let current_minus_two = cups_prev[&current_minus_one];
        let mut current_minus_two_next_four = cups_next.get_mut(&current_minus_two).unwrap();
        current_minus_two_next_four.remove(2);
        current_minus_two_next_four.remove(2);
        current_minus_two_next_four.push(current_next);
        current_minus_two_next_four.push(next_first);

        let current_minus_three = cups_prev[&current_minus_two];
        let mut current_minus_three_next_four = cups_next.get_mut(&current_minus_three).unwrap();
        current_minus_three_next_four.remove(3);
        current_minus_three_next_four.push(current_next);

        // println!("CLOSED");
        // dbg!(&cups_next);

        // Place the cups.
        *cups_prev.get_mut(&cups_next[&destination][0]).unwrap() = third;
        *cups_prev.get_mut(&first).unwrap() = destination;
        
        let mut destination_next_four = cups_next.get_mut(&destination).unwrap();
        let destination_first = destination_next_four[0];
        let destination_second = destination_next_four[1];
        let destination_third = destination_next_four[2];
        let destination_fourth = destination_next_four[3];
        destination_next_four.insert(0, third);
        destination_next_four.insert(0, second);
        destination_next_four.insert(0, first);
        destination_next_four.remove(4);
        destination_next_four.remove(4);
        destination_next_four.remove(4);

        let destination_minus_one = cups_prev[&destination];
        let mut destination_minus_one_next_four = cups_next.get_mut(&destination_minus_one).unwrap();
        destination_minus_one_next_four.insert(1, third);
        destination_minus_one_next_four.insert(1, second);
        destination_minus_one_next_four.insert(1, first);
        destination_minus_one_next_four.remove(4);
        destination_minus_one_next_four.remove(4);
        destination_minus_one_next_four.remove(4);

        let destination_minus_two = cups_prev[&destination_minus_one];
        let mut destination_minus_two_next_four = cups_next.get_mut(&destination_minus_two).unwrap();
        destination_minus_two_next_four.insert(2, second);
        destination_minus_two_next_four.insert(2, first);
        destination_minus_two_next_four.remove(4);
        destination_minus_two_next_four.remove(4);

        let destination_minus_three = cups_prev[&destination_minus_two];
        let mut destination_minus_three_next_four = cups_next.get_mut(&destination_minus_three).unwrap();
        destination_minus_three_next_four.insert(3, first);
        destination_minus_three_next_four.remove(4);

        let mut first_next_four = cups_next.get_mut(&first).unwrap();
        first_next_four.insert(2, destination_second);
        first_next_four.insert(2, destination_first);
        first_next_four.remove(4);
        first_next_four.remove(4);

        let mut second_next_four = cups_next.get_mut(&second).unwrap();
        second_next_four.insert(1, destination_third);
        second_next_four.insert(1, destination_second);
        second_next_four.insert(1, destination_first);
        second_next_four.remove(4);
        second_next_four.remove(4);
        second_next_four.remove(4);

        let mut third_next_four = cups_next.get_mut(&third).unwrap();
        third_next_four.insert(0, destination_fourth);
        third_next_four.insert(0, destination_third);
        third_next_four.insert(0, destination_second);
        third_next_four.insert(0, destination_first);
        third_next_four.remove(4);
        third_next_four.remove(4);
        third_next_four.remove(4);
        third_next_four.remove(4);

        // Select the next current cup.
        current = current_next;
    }

    cups_next
}

// fn rotate(cups: &VecDeque<usize>, moves: usize) -> VecDeque<usize> {
//     let mut cups = cups.clone();

//     let max_val = *cups.iter().max().unwrap();

//     for x in 0..moves {
//         println!("{}", x);
//         //// dbg!(&cups);

//         let current_cup = cups.pop_front().unwrap();
//         cups.push_back(current_cup);

//         let first = cups.pop_front().unwrap();
//         let second = cups.pop_front().unwrap();
//         let third = cups.pop_front().unwrap();

//         let mut target_cup = if current_cup != 1 {current_cup - 1} else {max_val};
//         while target_cup == first || target_cup == second || target_cup == third {
//             target_cup = if target_cup != 1 {target_cup - 1} else {max_val};
//         }

//         for (i, cup) in cups.iter().enumerate() {
//             if *cup == target_cup {
//                 cups.insert((i + 1) % max_val, third);
//                 cups.insert((i + 1) % max_val, second);
//                 cups.insert((i + 1) % max_val, first);
//                 break;
//             }
//         }

//         // loop {
//         //     let next_cup = cups.pop_front().unwrap();
//         //     if next_cup == target_cup {
//         //         cups.push_front(third);
//         //         cups.push_front(second);
//         //         cups.push_front(first);
//         //         cups.push_front(next_cup);
//         //         break;
//         //     }
//         //     cups.push_back(next_cup);
//         // }

//         // Get back to the proper next cup.
//         loop {
//             let cup = cups.pop_front().unwrap();
//             cups.push_back(cup);
//             if cup == current_cup {
//                 break;
//             }
//         }
//     }

//     cups
// }

fn extend_cups(cups: &Vec<usize>) -> Vec<usize> {
    let mut cups = cups.clone();
    let max_val = *cups.iter().max().unwrap();

    for i in (max_val+1)..=1000000 {
        cups.push(i);
    }

    cups
}

fn print_cups(cups: &HashMap<usize, Vec<usize>>) -> String {
    let mut index = 1;
    let mut result = String::new();
    loop {
        index = cups[&index][0];
        if index == 1 {
            break;
        }
        result.push_str(&index.to_string());
    }

    result
}

fn find_cups(cups: &HashMap<usize, Vec<usize>>) -> usize {
    cups[&1][0] * cups[&1][1]
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();
    let all_values = parse_input(&input);

    let (cups_next, cups_prev) = make_map(&all_values);
    println!("{}", print_cups(&rotate(&cups_next, &cups_prev, 100, all_values[0])));

    let (cups_next, cups_prev) = make_map(&extend_cups(&all_values));
    println!("{}", find_cups(&rotate(&cups_next, &cups_prev, 10000000, all_values[0])));
}
