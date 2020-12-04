const INPUT: &str = include_str!("../input/01.txt");

fn main() {
    let mut numbers = INPUT
        .trim()
        .split_whitespace()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    numbers.sort();
    for (i, n) in itertools::enumerate(&numbers) {
        for (j, m) in itertools::enumerate(&numbers[i..]) {
            match numbers[i + j..].binary_search(&(2020 - n - m)) {
                Ok(k) => {
                    println!("{}", (n * m * numbers[i + j + k]));
                }
                Err(_) => {}
            }
        }
    }
}
