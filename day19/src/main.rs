use either::Either;
use once_cell::sync::OnceCell;
use std::collections::VecDeque;
use util::read_input;

static RULES: OnceCell<Box<[Rule]>> = OnceCell::new();

#[derive(Clone, Debug)]
struct MatchContext {
    s: String,
    rules: Vec<usize>,
}

impl MatchContext {
    fn step(&mut self) -> Either<Box<[MatchContext]>, bool> {
        if self.s.is_empty() && self.rules.is_empty() {
            return Either::Right(true);
        } else if self.s.is_empty() {
            return Either::Right(false);
        } else if self.rules.is_empty() {
            return Either::Right(false);
        }

        match &RULES.get().unwrap()[self.rules.remove(0)] {
            Rule::Empty => {
                // Shouldn't find an empty rule.
                panic!("");
            }
            Rule::Value(c) => {
                if *c == self.s.remove(0) {
                    return Either::Left(Box::new([]));
                } else {
                    return Either::Right(false);
                }
            }
            Rule::Recursive(ors) => {
                let mut new_targets = Vec::new();
                for or in ors.iter() {
                    let mut rules = or.clone().into_vec();
                    rules.extend(self.rules.iter());
                    new_targets.push(MatchContext {
                        s: self.s.clone(),
                        rules,
                    });
                }
                return Either::Left(new_targets.into_boxed_slice());
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Rule {
    // Rules will only be empty at initialization. If any empty rules exist in production, we have
    // made a mistake.
    Empty,
    Value(char),
    // List of indices for the recursive rules.
    Recursive(Box<[Box<[usize]>]>),
}

impl Rule {
    fn matches(&self, s: &str) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(MatchContext {
            s: s.to_string(),
            rules: vec![0],
        });

        while !queue.is_empty() {
            let mut context = queue.pop_front().unwrap();
            match context.step() {
                Either::Left(contexts) => {
                    if contexts.is_empty() {
                        queue.push_back(context);
                    } else {
                        queue.extend(contexts.iter().cloned());
                    }
                }
                Either::Right(succeeded) => {
                    if succeeded {
                        return true;
                    }
                }
            }
        }

        false
    }
}

fn parse_input(input: &[String]) -> (Box<[Rule]>, Box<[&String]>) {
    let mut iter = input.iter();
    let mut rules = vec![Rule::Empty; 132];
    loop {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut rule_split = line.split(": ");
        let index = rule_split.next().unwrap();
        let rule_definition = rule_split.next().unwrap();
        let mut rule_chars = rule_definition.chars();
        rules[usize::from_str_radix(index, 10).unwrap()] = (match rule_chars.next().unwrap() {
            '"' => Rule::Value(rule_chars.next().unwrap()),
            _ => {
                let mut ors = Vec::new();
                let mut ors_definitions = rule_definition.split(" | ");
                for or in ors_definitions {
                    let mut recursives = Vec::new();
                    for index in or.split(' ') {
                        recursives.push(usize::from_str_radix(index, 10).unwrap());
                    }
                    ors.push(recursives.into_boxed_slice());
                }
                Rule::Recursive(ors.into_boxed_slice())
            }
        });
    }
    (
        rules.into_boxed_slice(),
        iter.collect::<Vec<_>>().into_boxed_slice(),
    )
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();

    let (rules, messages) = parse_input(&input);
    RULES.set(rules);

    println!(
        "{}",
        messages
            .iter()
            .filter(|m| {
                // println!("--------------");
                RULES.get().unwrap()[0].matches(m)
            })
            .count()
    );
}
