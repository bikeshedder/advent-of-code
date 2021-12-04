const INPUT: &str = include_str!("../input/02.txt");

fn main() {
    let lines = INPUT
        .trim()
        .lines()
        .map(str::split_whitespace)
        .map(|mut a| (a.next().unwrap(), a.next().unwrap().parse::<i32>().unwrap()))
        .collect::<Vec<_>>();
    let (pos, depth, _aim) = lines
        .iter()
        .fold((0, 0, 0), |(pos, depth, aim), &(cmd, value)| match cmd {
            "forward" => (pos + value, depth + value * aim, aim),
            "down" => (pos, depth, aim + value),
            "up" => (pos, depth, aim - value),
            _ => panic!("Invalid cmd: {}", cmd),
        });
    let answer = pos * depth;
    println!("{}", answer);
}
