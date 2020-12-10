use util::read_input;
use std::collections::HashMap;

fn jolt_differences(input: &[usize]) -> usize {
    let mut sorted = input.iter().collect::<Vec<_>>();
    sorted.sort();
    let mut prev = 0;
    let mut one_count = 0;
    let mut three_count = 1;
    for i in sorted {
        if i - 1 == prev {
            one_count += 1;
        } else if i - 3 == prev {
            three_count += 1;
        }
        prev = *i;
    }
    one_count * three_count
}

fn arrangements(input: &[usize]) -> usize {
    let mut cache: HashMap<usize, usize> = HashMap::new();
    cache.insert(0, 1);

    let mut sorted = input.iter().collect::<Vec<_>>();
    sorted.sort();

    let mut last = 0;

    for i in sorted {
        let mut val = 0;
        val += cache.get(&(i - 1)).unwrap_or(&0);
        if *i > 1 {
            val += cache.get(&(i - 2)).unwrap_or(&0);
        }
        if *i > 2 {
            val += cache.get(&(i - 3)).unwrap_or(&0)
        }
        cache.insert(*i, val);
        last = *i
    }

    cache[&last]
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<usize>(&args[1]).collect::<Vec<usize>>();

    println!("{}", jolt_differences(&input));
    println!("{}", arrangements(&input));
}
