use util::read_input;
use std::collections::HashSet;

fn sum_num_questions(input: &[String]) -> usize {
    let mut result = 0;
    let mut group = HashSet::new();
    for line in input {
        if line.is_empty() {
            result += group.len();
            group = HashSet::new();
        }
        group = group.union(&line.chars().collect::<HashSet<char>>()).map(|c| *c).collect();
    }
    result += group.len();
    result
}

fn sum_num_questions2(input: &[String]) -> usize {
    let mut result = 0;
    let mut group = HashSet::new();
    let mut new_group = true;
    for line in input {
        //dbg!(&group);
        if line.is_empty() {
            //dbg!(&group);
            result += group.len();
            group = HashSet::new();
            new_group = true;
        }
        else if new_group {
            group = line.chars().collect();
            new_group = false;
        } else {
            group = group.intersection(&line.chars().collect::<HashSet<char>>()).map(|c| *c).collect();
        }
    }

            //dbg!(&group);
    result += group.len();
    result
}


fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();

    println!("{}", sum_num_questions(&input));
    println!("{}", sum_num_questions2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_sum() {
        let input = [
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ];

        assert_eq!(sum_num_questions(&input), 11);
    }

    #[test]
    fn test_sum2() {
        let input = [
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ];

        assert_eq!(sum_num_questions2(&input), 6);
    }
}
