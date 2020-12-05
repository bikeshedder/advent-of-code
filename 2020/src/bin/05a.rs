use itertools::Itertools;

const INPUT: &str = include_str!("../input/05.txt");

fn main() {
    let solution = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .fold(0, |n, d| (n << 1) + (d == 'B' || d == 'R') as u64)
        })
        .sorted()
        .last()
        .unwrap();
    println!("{}", solution);
}
