const INPUT: &str = include_str!("../input/06.txt");

fn main() {
    let fishes = INPUT
        .trim()
        .split(',')
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut timers = [0; 9];
    for fish in fishes {
        timers[fish] += 1;
    }
    for _ in 0..256 {
        let creating = timers[0];
        for i in 0..8 {
            timers[i] = timers[i + 1];
        }
        timers[6] += creating;
        timers[8] = creating;
    }
    let answer: usize = timers.iter().sum();
    println!("{}", answer);
}
