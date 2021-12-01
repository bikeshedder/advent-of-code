use itertools::Itertools;

const INPUT: &str = include_str!("../input/01.txt");

fn main() {
    let numbers = INPUT
        .trim()
        .split_whitespace()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let answer = numbers
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();
    println!("{}", answer);
}
