use util::read_input;

fn seat_id(input: &str) -> usize {
    let mut chars = input.chars();
    let mut bin = String::new();
    for _ in 0..7 {
        bin.push(match chars.next().unwrap() {
            'F' => '0',
            'B' => '1',
            _ => ' '
        });
    }
    let row = usize::from_str_radix(&bin, 2).unwrap();
    let mut bin = String::new();
    for _ in 0..3 {
        bin.push(match chars.next().unwrap() {
            'L' => '0',
            'R' => '1',
            _ => ' '
        });
    }
    let column = usize::from_str_radix(&bin, 2).unwrap();

    row * 8 + column
}

fn highest_seat_id(input: &[String]) -> usize {
    input.iter().map(|val| seat_id(val)).max().unwrap()
}

fn my_seat(input: &[String]) -> usize {
    let mut seats = input.iter().map(|val| seat_id(val)).collect::<Vec<_>>();
    seats.sort();
    let mut prev_seat = None;
    for seat in seats {
        if let Some(prev) = prev_seat {
            if seat != prev + 1 {
                return seat - 1;
            }
        }
        prev_seat = Some(seat);
    }
    return 0;
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();

    println!("{}", highest_seat_id(&input));
    println!("{}", my_seat(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_seat_id() {
        let input = [
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ];

        assert_eq!(seat_id(&input[0]), 567);
        assert_eq!(seat_id(&input[1]), 119);
        assert_eq!(seat_id(&input[2]), 820);
    }

    #[test]
    fn test_highest_seat_id() {
        let input = [
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ];

        assert_eq!(highest_seat_id(&input), 820);
    }
}
