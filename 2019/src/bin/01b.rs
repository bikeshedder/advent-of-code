const INPUT: &str = include_str!("../input/01.txt");

fn fuel(mut mass: usize) -> usize {
    let mut fuel = 0;
    while mass > 0 {
        mass = std::cmp::max((mass as isize / 3) - 2, 0) as usize;
        fuel += mass;
    }
    fuel
}

fn main() {
    let output: usize = INPUT
        .trim()
        .split_whitespace()
        .map(|line| line.parse::<usize>().unwrap())
        .map(fuel)
        .sum();
    println!("{}", output);
}
