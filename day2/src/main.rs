use std::str::{Chars, FromStr};
use util::read_input;

struct Entry {
    c: char,
    min: usize,
    max: usize,
    password: String,
}

fn collect_chars_until_character(chars: &mut Chars, end: char) -> String {
    let mut result = String::new();
    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => {
                return result;
            }
        };
        if c == end {
            return result;
        }
        result.push(c);
    }
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let min = match usize::from_str(&collect_chars_until_character(&mut chars, '-')) {
            Ok(val) => val,
            Err(err) => {
                return Err(format!("{}", err));
            }
        };
        let max = match usize::from_str(&collect_chars_until_character(&mut chars, ' ')) {
            Ok(val) => val,
            Err(err) => {
                return Err(format!("{}", err));
            }
        };
        let c = match chars.next() {
            Some(c) => c,
            None => {
                return Err("No character found.".to_string());
            }
        };

        // Skip the ':' and the ' '.
        chars.next();
        chars.next();

        let password = chars.collect::<String>();

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
            if c == entry.c {
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
    use crate::{easy, hard, Entry};

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
