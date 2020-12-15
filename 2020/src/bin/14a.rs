use regex::Regex;
use std::str::FromStr;

const INPUT: &str = include_str!("../input/14.txt");

#[derive(Default)]
struct Mask {
    mask: u64,
    write: u64,
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        value & self.mask | self.write
    }
}

impl FromStr for Mask {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.chars().fold(Self::default(), |bm, c| Mask {
            mask: bm.mask << 1 | (c == 'X') as u64,
            write: bm.write << 1 | (c == '1') as u64,
        }))
    }
}

enum Instruction {
    Mask(Mask),
    Mem { addr: usize, value: u64 },
}

fn main() {
    let instruction_re = Regex::new(r"^(mask|mem)(?:\[(\d+)\])? = (\S+)$").unwrap();
    let program = INPUT
        .lines()
        .map(|line| {
            let c = instruction_re.captures(line).unwrap();
            match &c[1] {
                "mask" => Instruction::Mask(c[3].parse().unwrap()),
                "mem" => Instruction::Mem {
                    addr: c[2].parse().unwrap(),
                    value: c[3].parse().unwrap(),
                },
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();

    let mut mem = [0u64; 65536];
    let mut mask = Mask::default();
    for instruction in program {
        match instruction {
            Instruction::Mask(x) => mask = x,
            Instruction::Mem { addr, value } => mem[addr] = mask.apply(value),
        }
    }
    let solution = mem.iter().sum::<u64>();
    println!("{}", solution);
}
