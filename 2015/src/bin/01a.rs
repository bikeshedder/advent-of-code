const INPUT: &str = include_str!("../input/01.txt");

fn main() {
    let output = INPUT
        .trim()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!(),
        })
        .sum::<i64>();
    println!("{}", output);
}
