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
        .tuple_windows()
        .find(|(a, b)| *a + 2 == *b)
        .unwrap()
        .0
        + 1;
    println!("{}", solution);
}
