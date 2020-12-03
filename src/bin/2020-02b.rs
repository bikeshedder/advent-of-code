use regex::Regex;

const INPUT: &str = include_str!("2020-02.txt");

struct Line<'a> {
    min: usize,
    max: usize,
    c: char,
    s: &'a str,
}

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();
    let correct = INPUT
        .trim()
        .split("\n")
        .map(|line| {
            let m = re.captures(line).unwrap();
            Line {
                min: m[1].parse().unwrap(),
                max: m[2].parse().unwrap(),
                c: m[3].chars().next().unwrap(),
                s: m.get(4).unwrap().as_str(),
            }
        })
        .filter(|line| {
            let x = line.s.chars().nth(line.min - 1).unwrap();
            let y = line.s.chars().nth(line.max - 1).unwrap();
            match (x == line.c, y == line.c) {
                (true, false) | (false, true) => true,
                _ => false,
            }
        })
        .count();
    println!("{}", correct);
}
