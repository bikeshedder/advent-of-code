const INPUT: &str = include_str!("../input/01.txt");

fn main() {
    let mut numbers = INPUT
        .trim()
        .split_whitespace()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    numbers.sort();
    for (i, n) in itertools::enumerate(&numbers) {
        match numbers[i..].binary_search(&(2020 - n)) {
            Ok(j) => {
                println!("{}", (n * numbers[i + j]));
            }
            Err(_) => {}
        }
    }
}
