const INPUT: &str = include_str!("../input/03.txt");
const COLS: usize = 12;

fn main() {
    let (line_count, cols) =
        INPUT
            .trim()
            .split_whitespace()
            .fold((0, [0; COLS]), |(line_count, cols), line| {
                let mut chars = line.chars();
                (
                    line_count + 1,
                    cols.map(|n| n + (chars.next().unwrap() == '1') as i32),
                )
            });
    let gamma = cols
        .iter()
        .fold(0, |value, &n| (value << 1) + (n > line_count / 2) as i32);
    let epsilon = (1 << COLS) - gamma - 1;
    let answer = gamma * epsilon;
    println!("{}", answer);
}
