const INPUT: &str = include_str!("../input/10.txt");

fn main() {
    let mut adapters = INPUT
        .lines()
        .map(|line| str::parse::<i64>(line).unwrap())
        .collect::<Vec<_>>();
    adapters.sort();
    let mut paths = vec![0u64; adapters.len()];
    for (i, &adapter) in adapters.iter().enumerate() {
        if adapter <= 3 {
            paths[i] += 1;
        }
        if i >= 1 && adapter - adapters[i - 1] <= 3 {
            paths[i] += paths[i - 1];
        }
        if i >= 2 && adapter - adapters[i - 2] <= 3 {
            paths[i] += paths[i - 2];
        }
        if i >= 3 && adapter - adapters[i - 3] <= 3 {
            paths[i] += paths[i - 3];
        }
    }
    println!("{}", paths.last().unwrap());
}
