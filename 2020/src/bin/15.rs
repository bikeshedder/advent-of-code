use std::collections::{hash_map::Entry, HashMap};

const INPUT: &str = include_str!("../input/15.txt");

fn step(past: &mut HashMap<usize, usize>, n: usize, i: usize) -> usize {
    match past.entry(n) {
        Entry::Vacant(entry) => {
            entry.insert(i);
            0
        }
        Entry::Occupied(mut entry) => i - entry.insert(i),
    }
}

fn game(nth: usize) -> usize {
    let numbers: Vec<usize> = INPUT
        .trim()
        .split(",")
        .filter_map(|n| n.parse().ok())
        .collect();
    let mut past: HashMap<usize, usize> = HashMap::new();
    let mut next = 0usize;
    for (i, n) in numbers.iter().enumerate() {
        next = step(&mut past, *n, i);
    }
    for i in numbers.len()..nth - 1 {
        next = step(&mut past, next, i);
    }
    next
}

fn main() {
    println!("{}", game(2020));
    println!("{}", game(30000000));
}
