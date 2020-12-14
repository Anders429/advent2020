use std::collections::HashMap;
use std::str::FromStr;
use substring::Substring;
use util::read_input;

#[derive(Clone, Copy, Debug)]
struct Mask {
    zeros: u64,
    ones: u64,
}

impl Mask {
    const MAX_MASK: u64 = 68719476735;

    fn new(s: &str) -> Self {
        let mut zeros = Self::MAX_MASK;
        let mut ones = 0;

        for (i, c) in s.chars().enumerate() {
            match c {
                '0' => zeros &= Self::MAX_MASK - (1 << (35 - i)),
                '1' => ones |= 1 << (35 - i),
                _ => {}
            }
        }

        Self { zeros, ones }
    }

    fn apply(&self, val: u64) -> u64 {
        (val | self.ones) & self.zeros
    }

    fn floating(&self) -> u64 {
        (Self::MAX_MASK - self.ones) & self.zeros
    }

    fn apply_floating(&self, val: u64) -> Box<[u64]> {
        let mut masked_val = val | self.ones;

        let floating = self.floating();
        masked_val &= Self::MAX_MASK ^ floating;
        let mut floating_bits = Vec::new();

        for i in 0..36 {
            // Check if the bit should be floating.
            if (floating & (1 << i)) >> i == 1 {
                floating_bits.push(i);
            }
        }

        let mut result = Vec::new();

        // Iterate through each possible combination of 1s bits, and apply them.
        for i in 0..(1 << floating_bits.len()) {
            let mut new_val = masked_val.clone();
            let mut index = 0;
            for j in 0..36 {
                // Is this a one bit this time?
                if (i & (1u64 << j)) >> j == 1 {
                    new_val |= 1 << floating_bits[j];
                    index += 1;
                }
            }
            result.push(new_val);
        }

        result.into_boxed_slice()
    }
}

enum Instruction {
    Mask(Mask),
    // location, value.
    Mem(u64, u64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sides = s.split(" = ").collect::<Vec<_>>();
        match sides[0].substring(0, 4) {
            "mask" => Ok(Instruction::Mask(Mask::new(sides[1]))),
            "mem[" => Ok(Instruction::Mem(
                u64::from_str_radix(sides[0].substring(4, sides[0].len() - 1), 10).unwrap(),
                u64::from_str_radix(sides[1], 10).unwrap(),
            )),
            _ => Err(String::new()),
        }
    }
}

fn sum_mem(input: &[Instruction]) -> u64 {
    let mut mask = Mask::new("");
    let mut mem = HashMap::new();

    for instr in input {
        match instr {
            Instruction::Mask(m) => {
                mask = *m;
            }
            Instruction::Mem(loc, val) => {
                mem.insert(*loc, mask.apply(*val));
            }
        }
    }

    mem.values().sum()
}

fn sum_mem_floating(input: &[Instruction]) -> u64 {
    let mut mask = Mask::new("");
    let mut mem = HashMap::new();

    for instr in input {
        match instr {
            Instruction::Mask(m) => {
                mask = *m;
            }
            Instruction::Mem(loc, val) => {
                let masked_locs = mask.apply_floating(*loc);
                for masked_loc in masked_locs.iter() {
                    mem.insert(*masked_loc, *val);
                }
            }
        }
    }

    mem.values().sum()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Instruction>(&args[1]).collect::<Vec<Instruction>>();

    println!("{}", sum_mem(&input));
    println!("{}", sum_mem_floating(&input));
}
