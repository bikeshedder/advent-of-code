use itertools::Itertools;

const INPUT: &str = include_str!("../input/08.txt");

fn parse_bitmask(s: &str) -> u8 {
    s.chars().fold(0, |a, c| (a | (1 << (c as u8 - 'a' as u8))))
}

fn count_1_bits(v: &u8) -> u8 {
    let mut v = *v;
    let mut sum = 0;
    while v > 0 {
        sum += v & 1;
        v >>= 1;
    }
    sum
}

fn decode_digit(bitmask: &[u8; 10], digit: u8) -> u32 {
    bitmask.iter().find_position(|&&x| x == digit).unwrap().0 as u32
}

fn decode(bitmask: &[u8; 10], digits: &[u8]) -> u32 {
    digits.iter().fold(0, |mut acc, digit| {
        acc *= 10;
        acc += decode_digit(bitmask, *digit);
        acc
    })
}

fn solve_line(line: &'static str) -> u32 {
    let mut it = line.split(" | ");
    let mut signal_patterns = it
        .next()
        .unwrap()
        .split_whitespace()
        .map(parse_bitmask)
        .collect::<Vec<_>>();
    signal_patterns.sort_by_key(count_1_bits);
    let s1 = signal_patterns[0];
    let s7 = signal_patterns[1];
    let s4 = signal_patterns[2];
    let s8 = signal_patterns[9];
    let s235 = &signal_patterns[3..=5];
    let s069 = &signal_patterns[6..=8];
    // search for 3
    let s3 = *s235.iter().find(|&&p| p & s1 == s1).unwrap();
    // search for 6
    let s6 = *s069.iter().find(|&&p| p & s1 != s1).unwrap();
    // search for 9
    let s9 = *s069.iter().find(|&&p| p & s3 == s3).unwrap();
    // search for 0
    let s0 = *s069.iter().find(|&&p| p != s6 && p != s9).unwrap();
    // search for 5
    let s5 = *s235.iter().find(|&&p| p != s3 && p & s9 == p).unwrap();
    // search for 2
    let s2 = *s235.iter().find(|&&p| p != s3 && p != s5).unwrap();
    // combine in one array
    let bitmasks = [s0, s1, s2, s3, s4, s5, s6, s7, s8, s9];
    let output_values = it
        .next()
        .unwrap()
        .split_whitespace()
        .map(parse_bitmask)
        .collect::<Vec<_>>();
    decode(&bitmasks, &output_values)
}

fn main() {
    let answer = INPUT.trim().split("\n").map(solve_line).sum::<u32>();
    println!("{}", answer);
}
