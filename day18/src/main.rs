use util::read_input;
use std::str::Chars;

enum Mode {
    Start,
    Add,
    Mul,
}

fn evaluate(chars: &mut Chars) -> usize {
    let mut result = 0;
    let mut digit_str = String::new();
    let mut mode = Mode::Start;

    loop {
        if let Some(c) = chars.next() {
            if c.is_digit(10) {
                digit_str.push(c);
            } else if c == '+' {
                mode = Mode::Add;
            } else if c == '*' {
                mode = Mode::Mul;
            } else if c == '(' {
                match mode {
                    Mode::Start => {
                        result = evaluate(chars);
                    }
                    Mode::Add => {
                        result += evaluate(chars);
                    }
                    Mode::Mul => {
                        result *= evaluate(chars);
                    }
                }
            } else if c == ')' {
                match mode {
                    Mode::Start => {
                        return usize::from_str_radix(&digit_str, 10).unwrap_or(0);
                    }
                    Mode::Add => {
                        return result + usize::from_str_radix(&digit_str, 10).unwrap_or(0);
                    }
                    Mode::Mul => {
                        return result * usize::from_str_radix(&digit_str, 10).unwrap_or(1);
                    }
                }
            } else {
                match mode {
                    Mode::Start => {
                        if digit_str.is_empty() {
                            continue;
                        }
                        result = usize::from_str_radix(&digit_str, 10).unwrap();
                        digit_str = String::new();
                    }
                    Mode::Add => {
                        if digit_str.is_empty() {
                            continue;
                        }
                        result += usize::from_str_radix(&digit_str, 10).unwrap();
                        digit_str = String::new();
                    }
                    Mode::Mul => {
                        if digit_str.is_empty() {
                            continue;
                        }
                        result *= usize::from_str_radix(&digit_str, 10).unwrap();
                        digit_str = String::new();
                    }
                }
            }
        } else {
            match mode {
                Mode::Start => {
                    result = usize::from_str_radix(&digit_str, 10).unwrap_or(0);
                    digit_str = String::new();
                }
                Mode::Add => {
                    result += usize::from_str_radix(&digit_str, 10).unwrap_or(0);
                    digit_str = String::new();
                }
                Mode::Mul => {
                    result *= usize::from_str_radix(&digit_str, 10).unwrap_or(1);
                    digit_str = String::new();
                }
            }
            break;
        }
    }

    result
} 

/// This is a mess.
fn evaluate_2(chars: &mut Chars) -> usize {
    let original = chars.as_str();

    // Evaluate the addition only.
    let mut remaining_chars = String::new();

    let mut digit_str = String::new();
    let mut lhs = 0;
    let mut mode = Mode::Start;

    loop {
        if let Some(c) = chars.next() {
            //println!("{}, lhs={}, digit_str={}, remaining_chars={}", c, lhs, digit_str, remaining_chars);
            if c.is_digit(10) {
                digit_str.push(c);
            } else if c == '*' {
                if !digit_str.is_empty() {
                    lhs = usize::from_str_radix(&digit_str, 10).unwrap();
                }
                remaining_chars.push_str(&lhs.to_string());
                remaining_chars.push_str(" * ");
                lhs = 0;
            } else if c == '+' {
                mode = Mode::Add;
            } else if c == '(' {
                match mode {
                    Mode::Add => {
                        lhs += evaluate_2(chars);
                    }
                    _ => {
                        lhs = evaluate_2(chars);
                    }
                }
            } else if c == ')' {
                match mode {
                    Mode::Start => {
                        if digit_str.is_empty() {
                            remaining_chars.push_str(&lhs.to_string());
                            lhs = 0;
                        } else {
                            remaining_chars.push_str(&digit_str);
                        }
                        break;
                    }
                    Mode::Mul => {
                        remaining_chars.push_str(" * ");
                        if digit_str.is_empty() {
                            remaining_chars.push_str(&lhs.to_string());
                        } else {
                            remaining_chars.push_str(&digit_str);
                        }

                        break;
                    }
                    Mode::Add => {
                        remaining_chars.push_str(&(lhs + usize::from_str_radix(&digit_str, 10).unwrap_or(0)).to_string());
                        lhs = 0;
                        break;
                    }
                }
            } else {
                match mode {
                    Mode::Start => {
                        if digit_str.is_empty() {
                            continue;
                        }
                        lhs = usize::from_str_radix(&digit_str, 10).unwrap();
                        digit_str = String::new();
                    }
                    Mode::Mul => {
                        panic!("UHOH");
                        // Shouldn't get here.
                    }
                    Mode::Add => {
                        if digit_str.is_empty() {
                            continue;
                        }
                        lhs += usize::from_str_radix(&digit_str, 10).unwrap();
                        digit_str = String::new();
                        mode = Mode::Start;
                    }
                }
            }
        } else {
            match mode {
                Mode::Add => {
                    lhs += usize::from_str_radix(&digit_str, 10).unwrap_or(0);   
                }
                _ => {
                    if lhs == 0 {
                        lhs = usize::from_str_radix(&digit_str, 10).unwrap_or(0);
                    }
                }
            }
            remaining_chars.push_str(&lhs.to_string());
            break;
        }
    }

    //println!("lhs={}, digit_str={}, remaining_chars={}", lhs, digit_str, remaining_chars);

    println!("");
    println!("{}", original);
    println!("{}", remaining_chars);
    println!("{}", evaluate(&mut remaining_chars.chars()));

    evaluate(&mut remaining_chars.chars())

}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    // let input = read_input::<Expression>(&args[1]).collect::<Vec<Expression>>();
    
    // println!("{}", input.iter().map(|expr| expr.evaluate()).sum::<usize>())
    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();
    println!("{}", input.iter().map(|s| evaluate(&mut s.chars())).sum::<usize>());

    println!("{}", input.iter().map(|s| {println!(""); evaluate_2(&mut s.chars())}).sum::<usize>());
}
