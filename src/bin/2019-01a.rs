const INPUT: &str = include_str!("2019-01.txt");

fn main() {
    let output: usize = INPUT
        .trim()
        .split_whitespace()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|n| (n / 3) - 2)
        .sum();
    println!("{}", output);
}
