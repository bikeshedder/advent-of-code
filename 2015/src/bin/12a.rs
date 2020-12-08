use regex::Regex;

const INPUT: &str = include_str!("../input/12.txt");

fn main() {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let sum: i64 = re
        .find_iter(INPUT)
        .filter_map(|n| n.as_str().parse::<i64>().ok())
        .sum();
    println!("{}", sum);
}
