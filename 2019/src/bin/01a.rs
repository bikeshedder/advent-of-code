const INPUT: &str = include_str!("../input/01.txt");

fn main() {
    let output: usize = INPUT
        .trim()
        .split_whitespace()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|n| (n / 3) - 2)
        .sum();
    println!("{}", output);
}
