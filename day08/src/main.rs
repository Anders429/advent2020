use util::read_input;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl FromStr for Instruction {

    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let argument = isize::from_str_radix(parts[1], 10).unwrap();
        match parts[0] {
            "nop" => Ok(Self::Nop(argument)),
            "acc" => Ok(Self::Acc(argument)),
            "jmp" => Ok(Self::Jmp(argument)),
            _ => Ok(Self::Nop(argument)),
        }
    }
}

struct Console<'a> {
    instructions: &'a [Instruction],
}

impl<'a> Console<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
        }
    }

    fn run_until_loop(&self) -> isize {
        let mut accumulator = 0;
        let mut visited = HashSet::new();
        let mut index = 0;
        loop {
            if visited.contains(&index) {
                return accumulator
            }
            visited.insert(index.clone());
            let instr = &self.instructions[index];
            match instr {
                Instruction::Nop(_) => {index += 1;},
                Instruction::Acc(arg) => {accumulator += arg; index += 1},
                Instruction::Jmp(arg) => {if *arg > 0 {index += *arg as usize} else {index -= arg.abs() as usize}},
            } 
        }
    }

    fn does_loop(&self, i: &usize) -> Result<isize, ()> {
        let mut accumulator = 0;
        let mut visited = HashSet::new();
        let mut index = 0;
        loop {
            if visited.contains(&index) {
                return Err(())
            }
            visited.insert(index.clone());

            if index == self.instructions.len() {
                break;
            }

            let mut instr = self.instructions[index];

            if *i == index {
                instr = match instr {
                    Instruction::Nop(arg) => Instruction::Jmp(arg),
                    Instruction::Jmp(arg) => Instruction::Nop(arg),
                    _ => instr,
                };
            }

            match instr {
                Instruction::Nop(_) => {index += 1;},
                Instruction::Acc(arg) => {accumulator += arg; index += 1},
                Instruction::Jmp(arg) => {if arg > 0 {index += arg as usize} else {index -= arg.abs() as usize}},
            } 
        }
        Ok(accumulator)
    }

    fn find_all_jmp_and_nop_indices(&self) -> Box<[usize]> {
        let mut result = Vec::new();
        for (i, instr) in self.instructions.iter().enumerate() {
            match instr {
                Instruction::Nop(_) | Instruction::Jmp(_) => result.push(i),
                _ => {},
            }
        }
        result.into_boxed_slice()
    }

    fn fix(&self) -> isize {
        let indices = self.find_all_jmp_and_nop_indices();

        for i in indices.iter() {
            match self.does_loop(i) {
                Ok(val) => {return val},
                Err(_) => {continue;}
            }
        }
        panic!("");
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Instruction>(&args[1]).collect::<Vec<Instruction>>();

    let mut c = Console::new(&input);

    println!("{}", c.run_until_loop());
    println!("{}", c.fix());
}

#[cfg(test)]
mod tests {
    use crate::*;
}
