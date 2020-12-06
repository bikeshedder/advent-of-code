use itertools::Itertools;

const INPUT: &str = include_str!("../input/06.txt");

fn main() {
    let solution = INPUT
        .split("\n\n")
        .map(|g| g.chars().filter(|&c| 'a' <= c && c <= 'z').unique().count())
        .sum::<usize>();
    println!("{}", solution);
}
