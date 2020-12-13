use std::str::FromStr;
use util::read_input;

enum Bus {
    OutOfService,
    InService(usize),
}

impl FromStr for Bus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match usize::from_str_radix(s, 10) {
            Ok(val) => Ok(Bus::InService(val)),
            Err(_) => Ok(Bus::OutOfService),
        }
    }
}

fn parse_input(input: &[String]) -> (usize, Box<[Bus]>) {
    let time = usize::from_str_radix(&input[0], 10).unwrap();
    let busses = input[1]
        .split(',')
        .map(|val| Bus::from_str(val).unwrap())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    (time, busses)
}

fn earliest(time: usize, busses: &[Bus]) -> usize {
    let mut best_diff = usize::MAX;
    let mut best_id = 0;

    for (i, bus) in busses.iter().enumerate() {
        match bus {
            Bus::InService(val) => {
                let diff = *val - (time % *val);
                println!("{} {}", *val, time % *val);
                if diff < best_diff {
                    best_diff = diff;
                    best_id = *val;
                }
            }
            _ => {}
        }
    }

    println!("{} {}", best_diff, best_id);

    best_diff * best_id
}

fn chinese_remainder_theorem(busses: &[Bus]) -> usize {
    let mut operating = busses
        .iter()
        .enumerate()
        .filter(|(_, bus)| matches!(bus, Bus::InService(_)))
        .map(|(i, bus)| {
            if let Bus::InService(val) = bus {
                (i, *val)
            } else {
                (i, 0)
            }
        })
        .map(|(a, n)| (a % n, n))
        .collect::<Vec<_>>();

    let mut x = operating[0].0;
    let mut N = operating[0].1;

    for i in 0..(operating.len() - 1) {
        while x % operating[i + 1].1 != (operating[i + 1].1 - operating[i + 1].0)
            || x < operating[i + 1].1
        {
            x += N;
        }
        N *= operating[i + 1].1;

        println!(
            "ai={} ni={} ai+1={} ni+1={} x={} N={}",
            operating[i].0,
            operating[i].1,
            operating[i + 1].0,
            operating[i + 1].1,
            x,
            N
        );
    }

    x
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();

    let (time, busses) = parse_input(&input);

    println!("{}\n", earliest(time, &busses));
    println!("{}", chinese_remainder_theorem(&busses));
}
