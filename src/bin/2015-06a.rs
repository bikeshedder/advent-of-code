use regex::Regex;

const INPUT: &str = include_str!("2015-06.txt");

fn main() {
    let re = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    let mut map = [[false; 1000]; 1000];
    for line in INPUT.trim().split("\n") {
        let m = re.captures(line).unwrap();
        let f = match &m[1] {
            "turn on" => |_| true,
            "turn off" => |_| false,
            "toggle" => |v: bool| !v,
            _ => unreachable!(),
        };
        let ax = m[2].parse::<usize>().unwrap();
        let ay = m[3].parse::<usize>().unwrap();
        let bx = m[4].parse::<usize>().unwrap();
        let by = m[5].parse::<usize>().unwrap();
        for x in ax..=bx {
            for y in ay..=by {
                let v = map[y][x];
                map[y][x] = f(v);
            }
        }
    }
    let output: usize = map
        .iter()
        .map(|row| row.iter().filter(|x| **x).count())
        .sum();
    println!("{:?}", output);
}
