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
            let count = line.s.chars().filter(|c| *c == line.c).count();
            line.min <= count && count <= line.max
        })
        .count();
    println!("{}", correct);
}
