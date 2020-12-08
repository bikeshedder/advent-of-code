use regex::Regex;

const INPUT: &str = include_str!("../input/14.txt");
const SECONDS: u64 = 2503;

fn main() {
    let re = Regex::new(
        r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$",
    )
    .unwrap();
    let solution = INPUT
        .lines()
        .filter_map(|line| {
            re.captures(line).map(|c| {
                (
                    c.get(1).unwrap().as_str(),
                    c[2].parse::<u64>().unwrap(),
                    c[3].parse::<u64>().unwrap(),
                    c[4].parse::<u64>().unwrap(),
                )
            })
        })
        .map(|(_name, speed, duration, rest)| {
            let cycles = SECONDS / (duration + rest);
            let remaining = SECONDS % (duration + rest);
            cycles * speed * duration + std::cmp::min(remaining, duration) * speed
        })
        .max()
        .unwrap();
    println!("{}", solution);
}
