const INPUT: &str = include_str!("../input/03.txt");
const COLS: usize = 12;

fn rating(mut numbers: &[i32], cmp: fn(&usize, &usize) -> bool) -> i32 {
    for bit in (0..COLS).rev() {
        if numbers.len() == 1 {
            break;
        }
        let idx = numbers.iter().position(|n| n >> bit & 1 == 1);
        if let Some(idx) = idx {
            let middle = numbers.len() / 2;
            if cmp(&idx, &middle) {
                numbers = &numbers[..idx];
            } else {
                numbers = &numbers[idx..];
            }
        }
    }
    assert!(numbers.len() == 1);
    *numbers.first().unwrap()
}

fn main() {
    let mut numbers = INPUT
        .trim()
        .split_whitespace()
        .map(|line| i32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();
    numbers.sort_unstable();
    let oxygen = rating(&numbers, usize::gt);
    let co2 = rating(&numbers, usize::le);
    let answer = oxygen * co2;
    println!("{}", answer);
}
