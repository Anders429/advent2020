use std::collections::HashMap;
use util::read_input;

#[derive(Debug)]
struct Bound {
    lower: usize,
    upper: usize,
}

#[derive(Debug)]
struct Field<'a> {
    name: &'a str,
    bounds: Box<[Bound]>,
}

impl<'a> Field<'a> {
    fn from_str(s: &'a str) -> Self {
        let mut split_1 = s.split(": ");
        let name = split_1.next().unwrap();
        let mut split_2 = split_1.next().unwrap().split(" or ");
        let mut bounds = Vec::new();
        for bound in split_2 {
            let mut split_3 = bound.split('-');
            let lower = usize::from_str_radix(split_3.next().unwrap(), 10).unwrap();
            let upper = usize::from_str_radix(split_3.next().unwrap(), 10).unwrap();
            bounds.push(Bound { lower, upper });
        }
        Field {
            name,
            bounds: bounds.into_boxed_slice(),
        }
    }

    fn check_bounds(&self, val: usize) -> bool {
        for bound in self.bounds.iter() {
            if val >= bound.lower && val <= bound.upper {
                return true;
            }
        }
        false
    }
}

fn parse_input(input: &[String]) -> (Box<[Field]>, Box<[usize]>, Box<[Box<[usize]>]>) {
    let mut iter = input.iter();
    let mut fields = Vec::new();
    loop {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }
        fields.push(Field::from_str(line));
    }
    // Your ticket
    iter.next();
    let own_ticket = iter
        .next()
        .unwrap()
        .split(',')
        .map(|i| usize::from_str_radix(i, 10).unwrap())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    iter.next();
    iter.next();

    let mut tickets = Vec::new();
    loop {
        if let Some(ticket_raw) = iter.next() {
            tickets.push(
                ticket_raw
                    .split(',')
                    .map(|i| usize::from_str_radix(i, 10).unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        } else {
            break;
        }
    }

    (
        fields.into_boxed_slice(),
        own_ticket,
        tickets.into_boxed_slice(),
    )
}

fn error_rate(fields: &[Field], tickets: &[Box<[usize]>]) -> usize {
    let mut result = 0;

    for ticket in tickets {
        for entry in ticket.iter() {
            let mut found = false;
            for field in fields {
                if field.check_bounds(*entry) {
                    found = true;
                    break;
                }
            }
            if !found {
                result += entry;
            }
        }
    }

    result
}

fn remove_invalid(fields: &[Field], tickets: &[Box<[usize]>]) -> Box<[Box<[usize]>]> {
    let mut result = Vec::new();

    for ticket in tickets {
        let mut invalid = false;
        'entries: for entry in ticket.iter() {
            let mut valid = false;
            for field in fields {
                if field.check_bounds(*entry) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                invalid = true;
                break;
            }
        }
        if !invalid {
            result.push(
                ticket
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }
    }

    result.into_boxed_slice()
}

fn departure_product(fields: &[Field], own_ticket: &[usize], tickets: &[Box<[usize]>]) -> usize {
    let mut possible = HashMap::new();
    for field in fields {
        possible.insert(
            field.name.to_string(),
            (0..own_ticket.len()).collect::<Vec<_>>(),
        );
    }

    dbg!(tickets.len());

    'running: for ticket in tickets {
        // Check for end condition.
        let mut end = true;
        for p in possible.values() {
            if p.len() > 1 {
                end = false;
                break;
            }
        }
        if end {
            break;
        }

        //dbg!(&ticket);
        for (i, entry) in ticket.iter().enumerate() {
            for field in fields {
                if !possible[field.name].contains(&i) {
                    continue;
                }

                if !field.check_bounds(*entry) {
                    println!("Removing {}", i);
                    dbg!(field);
                    dbg!(entry);
                    dbg!(i);
                    possible.get_mut(field.name).unwrap().retain(|&x| x != i);
                }
            }
        }
    }

    loop {
        // End condition
        let mut end = true;
        for value in possible.values() {
            if value.len() != 1 {
                end = false;
                break;
            }
        }
        if end {
            break;
        }

        for key in possible.clone().keys() {
            if possible[key].len() == 1 {
                let val = possible[key][0];
                // Remove the value from the other lists.
                for other_key in possible.clone().keys() {
                    if key == other_key {
                        continue;
                    }
                    possible.get_mut(other_key).unwrap().retain(|&x| x != val);
                }
            }
        }
    }

    dbg!(&possible);

    let mut result = 1;

    for key in possible.keys() {
        if key.starts_with("departure") {
            result *= own_ticket[possible[key][0]];
        }
    }

    result
    //own_ticket[possible["seat"][0]]
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();

    let (fields, own_ticket, tickets) = parse_input(&input);

    println!("{}", error_rate(&fields, &tickets));

    let valid_tickets = remove_invalid(&fields, &tickets);
    println!(
        "{}",
        departure_product(&fields, &own_ticket, &valid_tickets)
    );
}
