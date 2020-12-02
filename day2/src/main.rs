use std::str::FromStr;
use util::read_input;

struct Entry {
    c: char,
    min: usize,
    max: usize,
    password: String,
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut min_str = String::new();
        loop {
            let c = chars.next().unwrap();
            if c == '-' {
                break;
            }
            min_str.push(c);
        }
        let min = usize::from_str(&min_str).unwrap();
        let mut max_str = String::new();
        loop {
            let c = chars.next().unwrap();
            if c == ' ' {
                break;
            }
            max_str.push(c);
        }
        let max = usize::from_str(&max_str).unwrap();

        let c = chars.next().unwrap();
        chars.next();
        chars.next();

        let mut password = String::new();
        loop {
            let ch = match chars.next() {
                Some(ch) => ch,
                None => {break;}
            };
            password.push(ch);
        }

        Ok(Self {
            c,
            min,
            max,
            password,
        })
    }
}

fn easy(input: &[Entry]) -> usize {
    let mut result = 0;
    for entry in input {
        let mut count = 0;
        for c in entry.password.chars() {
            if c ==  entry.c {
                count += 1;
            }
        }
        if count >= entry.min && count <= entry.max {
            result += 1;
        }
    }
    result
}

fn hard(input: &[Entry]) -> usize {
    let mut result = 0;
    for entry in input {
        let mut found = false;
        for (i, c) in entry.password.char_indices() {
            if i + 1 == entry.min && entry.c == c {
                found = true;
            } else if i + 1 == entry.max && entry.c == c {
                if found {
                    found = false;
                    break;
                }
                found = true;
            }
        }
        if found {
            result += 1;
        }
    }
    result
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Entry>(&args[1]).collect::<Vec<Entry>>();

    println!("{}", easy(&input));
    println!("{}", hard(&input));
}

#[cfg(test)]
mod tests {
    use crate::Entry;
    use crate::easy;
    use crate::hard;

    #[test]
    fn test_easy() {
        let input = &[
            Entry {
                c: 'a',
                min: 1,
                max: 3,
                password: "abcde".to_string(),
            },
            Entry {
                c: 'b',
                min: 1,
                max: 3,
                password: "cdefg".to_string(),
            },
            Entry {
                c: 'c',
                min: 2,
                max: 9,
                password: "ccccccccc".to_string(),
            },
        ];

        assert_eq!(easy(input), 2);
    }

    #[test]
    fn test_hard() {
        let input = &[
            Entry {
                c: 'a',
                min: 1,
                max: 3,
                password: "abcde".to_string(),
            },
            Entry {
                c: 'b',
                min: 1,
                max: 3,
                password: "cdefg".to_string(),
            },
            Entry {
                c: 'c',
                min: 2,
                max: 9,
                password: "ccccccccc".to_string(),
            },
        ];

        assert_eq!(hard(input), 1);
    }
}
