use itertools::Itertools;

const INPUT: &str = include_str!("2015-05.txt");

const VOWELS: &'static str = "aeiou";
const BAD_WORDS: &'static [&str] = &["ab", "cd", "pq", "xy"];

fn main() {
    let output = INPUT
        .trim()
        .split("\n")
        // at least three vowels
        .filter(|w| w.chars().filter(|c| VOWELS.find(*c).is_some()).count() >= 3)
        // at least one letter twice in a row
        .filter(|w| w.chars().tuple_windows().any(|(a, b)| a == b))
        // contains no bad words
        .filter(|w| !BAD_WORDS.iter().any(|&b| w.find(b).is_some()))
        .count();
    println!("{:?}", output);
}
