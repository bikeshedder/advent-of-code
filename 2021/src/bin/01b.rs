const INPUT: &str = include_str!("../input/01.txt");

fn main() {
    let numbers = INPUT
        .trim()
        .split_whitespace()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let sums = numbers
        .windows(3)
        .map(|w| w.iter().sum::<i64>())
        .collect::<Vec<_>>();
    let answer = sums
        .windows(2)
        .map(|w| w.get(1).unwrap() > w.get(0).unwrap())
        .filter(|b| *b)
        .count();
    println!("{}", answer);
}
