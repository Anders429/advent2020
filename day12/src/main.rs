use std::str::FromStr;
use util::read_input;

#[derive(Debug)]
enum Instruction {
    N(usize),
    S(usize),
    E(usize),
    W(usize),
    L(usize),
    R(usize),
    F(usize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let c = chars.next().unwrap();
        let val = usize::from_str_radix(&chars.collect::<String>(), 10).unwrap();
        match c {
            'N' => Ok(Self::N(val)),
            'S' => Ok(Self::S(val)),
            'E' => Ok(Self::E(val)),
            'W' => Ok(Self::W(val)),
            'L' => Ok(Self::L(val)),
            'R' => Ok(Self::R(val)),
            'F' => Ok(Self::F(val)),
            _ => Err("Invalid character".to_string()),
        }
    }
}

fn manhattan_distance(input: &[Instruction]) -> usize {
    let mut x: isize = 0;
    let mut y: isize = 0;

    // East is initial direction.
    let mut direction = 0;

    for i in input {
        match i {
            Instruction::N(val) => {
                y += *val as isize;
            }
            Instruction::S(val) => {
                y -= *val as isize;
            }
            Instruction::E(val) => {
                x += *val as isize;
            }
            Instruction::W(val) => {
                x -= *val as isize;
            }
            Instruction::L(val) => {
                direction += *val;
                direction %= 360;
            }
            Instruction::R(val) => {
                if *val > direction {
                    let mut c = val - direction;
                    c %= 360;
                    direction = 360 - c;
                } else {
                    direction -= val;
                }
            }
            Instruction::F(val) => match direction {
                0 => {
                    x += *val as isize;
                }
                90 => {
                    y += *val as isize;
                }
                180 => {
                    x -= *val as isize;
                }
                270 => {
                    y -= *val as isize;
                }
                _ => {
                    panic!("Unexpected direction.")
                }
            },
        }
    }

    (x.abs() + y.abs()) as usize
}

fn waypoint(input: &[Instruction]) -> usize {
    let mut x: isize = 10;
    let mut y: isize = 1;

    let mut x2: isize = 0;
    let mut y2: isize = 0;

    for i in input {
        dbg!(i);
        match i {
            Instruction::N(val) => {
                y += *val as isize;
            }
            Instruction::S(val) => {
                y -= *val as isize;
            }
            Instruction::E(val) => {
                x += *val as isize;
            }
            Instruction::W(val) => {
                x -= *val as isize;
            }
            Instruction::L(val) => {
                match *val {
                    90 => {
                        let c = x;
                        x = -y;
                        y = c;
                    },
                    180 => {
                        x = -x;
                        y = -y;
                    },
                    270 => {
                        let c = x;
                        x = y;
                        y = -c;
                    },
                    360 => {},
                    _ => {
                    panic!("Unexpected direction.")
                }
                }
            }
            Instruction::R(val) => {
                match *val {
                    90 => {
                        let c = x;
                        x = y;
                        y = -c;
                    },
                    180 => {
                        x = -x;
                        y = -y;
                    },
                    270 => {
                        let c = x;
                        x = -y;
                        y = c;
                    },
                    360 => {},
                    _ => {
                    panic!("Unexpected direction.")
                }
                }
            }
            Instruction::F(val) => {
                x2 += *val as isize * x;
                y2 += *val as isize * y;
            }
        }

        println!("position: {} {}", x2, y2);
        println!("waypoint: {} {}", x, y);
    }

    (x2.abs() + y2.abs()) as usize
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Instruction>(&args[1]).collect::<Vec<Instruction>>();

    println!("{}", manhattan_distance(&input));
    println!("{}", waypoint(&input));
}
