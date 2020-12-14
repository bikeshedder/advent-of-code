use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("../input/14.txt");

#[derive(Debug, Default)]
struct Mask {
    overwrite: u64,
    floating_mask: u64,
    floating_bits: Vec<[u64; 2]>,
}

impl Mask {
    fn get_addresses(&self, addr: u64) -> impl Iterator<Item = u64> + '_ {
        let addr = (addr | self.overwrite) & !self.floating_mask;
        self.floating_bits
            .iter()
            .multi_cartesian_product()
            .map(move |p| p.iter().fold(addr, |v, f| v | *f))
    }
}

impl FromStr for Mask {
    type Err = char;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = Mask::default();
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '0' => {}
                '1' => mask.overwrite |= 1 << i,
                'X' => {
                    mask.floating_mask |= 1 << i;
                    mask.floating_bits.push([0, 1 << i]);
                }
                c => return Err(c),
            }
        }
        Ok(mask)
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    Mem { addr: u64, value: u64 },
}

#[derive(Default)]
struct Memory {
    data: HashMap<u64, u64>,
}

impl Memory {
    fn iter(&self) -> impl Iterator<Item = &u64> {
        self.data.values()
    }
}

impl std::ops::Index<u64> for Memory {
    type Output = u64;
    fn index(&self, index: u64) -> &Self::Output {
        self.data.get(&index).unwrap_or(&0)
    }
}

impl std::ops::IndexMut<u64> for Memory {
    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        self.data.entry(index).or_insert(0)
    }
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
    let mut mem = Memory::default();
    let mut mask = Mask::default();
    for instruction in program {
        match instruction {
            Instruction::Mask(x) => mask = x,
            Instruction::Mem { addr, value } => {
                for addr in mask.get_addresses(addr) {
                    mem[addr] = value;
                }
            }
        }
    }
    let solution = mem.iter().map(|x| *x).sum::<u64>();
    println!("{}", solution);
}
