use itertools::Itertools;

const INPUT: &str = include_str!("../input/09.txt");
const INVALID_NUMBER: u64 = 31161678;

fn main() {
    let lines: Vec<u64> = INPUT
        .lines()
        .map(|line| str::parse(line).unwrap())
        .collect();
    let solution = (0..lines.len())
        .filter_map(|i| {
            (i + 1..lines.len())
                .scan(lines[i], |n, j| {
                    if *n < INVALID_NUMBER {
                        *n += lines[j];
                        Some((*n, j))
                    } else {
                        None
                    }
                })
                .filter_map(|(n, j)| {
                    if n == INVALID_NUMBER {
                        let (a, b) = lines[i..j].iter().minmax().into_option().unwrap();
                        Some(a + b)
                    } else {
                        None
                    }
                })
                .next()
        })
        .next()
        .unwrap();
    println!("{}", solution);
}
