use util::read_input;
use std::collections::VecDeque;
use std::collections::HashMap;

fn find(input: &[usize], preamble: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut iter = input.iter();
    iter.cloned().take(preamble).for_each(|i | queue.push_back(i));

    for i in input[preamble..input.len()].iter() {
        let mut found = false;
        for (j_index, j) in queue.iter().enumerate() {
            for (k_index, k) in queue.iter().enumerate() {
                if j_index == k_index {
                    continue;
                }
                if *j + *k == *i {
                    found = true;
                }
            }
        }
        queue.pop_front();
        queue.push_back(*i);
        if !found {
            return *i;
        }
    }
    0
}

fn contiguous(input: &[usize], target: usize) -> usize {
    for i in 0..input.len() {
        let mut sum = 0;
        //println!("");
        for j in i..input.len() {
            //println!("{}", input[j]);
            sum += input[j];
            if sum == target {
                let mut cont = input[i..=j].iter().cloned().collect::<Vec<_>>();
                cont.sort();
                return cont[0] + cont[cont.len() - 1];
            }
        }
    }
    0
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<usize>(&args[1]).collect::<Vec<usize>>();

    println!("{}", find(&input, 25));
    println!("{}", contiguous(&input, 258585477));
}
