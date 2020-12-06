const INPUT: &str = include_str!("../input/06.txt");

fn main() {
    let solution = INPUT
        .split("\n\n")
        .map(|g| {
            let mut cnt = 0;
            let mut chars = [0; 26];
            for line in g.lines() {
                cnt += 1;
                for c in line.chars() {
                    chars[c as usize - 'a' as usize] += 1;
                }
            }
            chars.iter().filter(|x| **x == cnt).count()
        })
        .sum::<usize>();
    println!("{}", solution);
}
