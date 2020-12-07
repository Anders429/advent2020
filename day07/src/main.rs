use util::read_input;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

fn get_bag_tree_2(input: &[String]) -> HashMap<String, HashMap<String, usize>> {
    let mut result = HashMap::new();

    for line in input {
        let mut chars = line.chars();
        let mut space_count = 0;
        let mut root = String::new();
        while space_count < 2 {
            let c = chars.next().unwrap();
            if c == ' ' {
                space_count += 1;
                if space_count != 2 {
                    root.push(c);
                }
                continue;
            }
            root.push(c);
        }
        space_count = 0;
        while space_count < 2 {
            if chars.next().unwrap() == ' ' {
                space_count += 1;
                continue;
            }
        }

        let remainder = chars.collect::<String>();
        let mut inner_map = HashMap::new();
        for entry in remainder.split(", ") {
            if entry == "no other bags." {
                break;
            }

            let mut chars = entry.chars();
            let mut spaces = 0;
            let mut num_str = String::new();
            while spaces < 1 {
                let c = chars.next().unwrap();
                if c == ' ' {
                    spaces += 1;
                    continue;
                }
                num_str.push(c);
            }
            let num = usize::from_str_radix(&num_str, 10).unwrap();
            spaces = 0;
            let mut bag = String::new();
            while spaces < 2 {
                let c = chars.next().unwrap();
                if c == ' ' {
                    spaces += 1;
                    if spaces != 2 {
                        bag.push(c);
                    }
                    continue;
                }
                bag.push(c);
            }
            inner_map.insert(bag, num);
        }
        result.insert(root, inner_map);
    }

    result
}

fn get_bag_tree(input: &[String]) -> HashMap<String, HashMap<String, usize>> {
    let re = Regex::new(r"(?P<root_bag>([a-z]+ ){2}) bags contain(?P<contained> (?P<num>\d+) (?P<bag>([a-z]+ ){2}) bags,?)+").unwrap();
    let inner_re = Regex::new(r"(?P<num>\d+) (?P<bag>([a-z]+ ){2}) bags").unwrap();

    let mut result = HashMap::new();
    for line in input {
        let caps = re.captures(&line).unwrap();
        let root = &caps["root_bag"].to_string();
        let contained = &caps["contained"];

        let mut inner_map = HashMap::new();
        for inner in contained.split(",") {
            let inner_caps = inner_re.captures(&inner).unwrap();
            let num = &inner_caps["num"];
            let bag = &inner_caps["bag"].to_string();
            inner_map.insert(bag.clone(), usize::from_str_radix(num, 10).unwrap());
        }
        result.insert(root.clone(), inner_map);
    }

    result
}

fn contains_gold_bag(m: &HashMap<String, HashMap<String, usize>>, key: &str, visited: &mut HashSet<String>) -> bool {
    let inner = m.get(key).unwrap();
    for inner_key in inner.keys() {
        if inner_key == "shiny gold" {
            return true;
        }
        if visited.contains(inner_key) {
            continue;
        }
        visited.insert(inner_key.clone());
        if contains_gold_bag(m, &inner_key, visited) {
            return true;
        }
    }
    return false
}

fn find_gold_bag_count(input: &[String]) -> usize {
    let m = get_bag_tree_2(input);

    let mut result = 0;

    for key in m.keys() {
        if key == "shiny gold" {
            continue;
        }
        if contains_gold_bag(&m, key, &mut HashSet::new()) {
            result += 1;
        }

    }
    result
}

fn count_internal_bags(m: &HashMap<String, HashMap<String, usize>>, key: &str) -> usize {
    let mut result = 0;
    let inner_map = m.get(key).unwrap();
    for inner_key in inner_map.keys() {
        let count = count_internal_bags(m, inner_key);
        result += inner_map.get(inner_key).unwrap();
        result += count * inner_map.get(inner_key).unwrap();
    }
    result
}

fn find_count_within_gold_bag(input: &[String]) -> usize {
    let m = get_bag_tree_2(input);

    count_internal_bags(&m, "shiny gold")
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();

    println!("{}", find_gold_bag_count(&input));
    println!("{}", find_count_within_gold_bag(&input));
}

#[cfg(test)]
mod tests {

}
