use itertools::Itertools;

const INPUT: &str = include_str!("../input/10.txt");

fn main() {
    let output = (0..50).fold(INPUT.trim().to_owned(), |s, _| {
        s.chars()
            .group_by(|&c| c)
            .into_iter()
            .map(|(k, g)| format!("{}{}", g.count(), k))
            .join("")
    });
    println!("{}", output.len());
}
