const INPUT: &str = include_str!("2015-08.txt");

fn encode(line: &str) -> usize {
    line.chars()
        .map(|c| match c {
            '"' => 2,
            '\\' => 2,
            _ => 1,
        })
        .sum::<usize>()
        + 2
}

fn main() {
    let output = INPUT
        .trim()
        .split("\n")
        .map(|l| encode(l) - l.len())
        .sum::<usize>();
    println!("{}", output);
}
