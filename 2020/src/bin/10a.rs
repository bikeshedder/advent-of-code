use itertools::Itertools;

const INPUT: &str = include_str!("../input/10.txt");

fn main() {
    let (count, count3) = INPUT
        .lines()
        .map(|line| str::parse::<i64>(line).unwrap())
        .sorted()
        .tuple_windows()
        .fold((1, 1), |(count1, count3), (a, b)| {
            let x = b - a;
            (count1 + ((x == 1) as i64), count3 + ((x == 3) as i64))
        });
    let solution = count * count3;
    println!("{}", solution);
}
