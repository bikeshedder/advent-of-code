const INPUT: &str = include_str!("../input/01.txt");

fn main() {
    let numbers = INPUT
        .trim()
        .split_whitespace()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let answer = numbers
        .windows(2)
        .map(|w| w.get(1).unwrap() > w.get(0).unwrap())
        .filter(|b| *b)
        .count();
    println!("{}", answer);
}
