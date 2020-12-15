use std::collections::HashMap;

const INPUT: &str = include_str!("../input/15.txt");

fn game(nth: usize) -> usize {
    let numbers: Vec<usize> = INPUT
        .trim()
        .split(",")
        .filter_map(|n| n.parse().ok())
        .collect();
    let mut past: HashMap<usize, usize> = HashMap::new();
    for (i, n) in numbers.iter().enumerate() {
        past.insert(*n, i);
    }
    let mut current = 0usize;
    let mut next = 0usize;
    for i in numbers.len()..nth {
        current = next;
        next = match past.get(&next) {
            Some(j) => i - j,
            None => 0
        };
        past.insert(current, i);
    }
    current
}

fn main() {
    println!("{}", game(2020));
    println!("{}", game(30000000));
}
