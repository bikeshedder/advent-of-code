const INPUT: &str = include_str!("../input/15.txt");

fn step(history: &mut [u32], n: u32, i: u32) -> u32 {
    let next = match history[n as usize] {
        std::u32::MAX => 0,
        j => i - j
    };
    history[n as usize] = i;
    next
}

fn game(nth: u32) -> u32 {
    let start: Vec<u32> = INPUT
        .trim()
        .split(",")
        .filter_map(|n| n.parse().ok())
        .collect();
    let mut history = vec![std::u32::MAX; nth as usize];
    let mut next = 0u32;
    for (n, i) in start.iter().zip(0u32..) {
        next = step(&mut history, *n, i);
    }
    for i in start.len() as u32..nth - 1 {
        next = step(&mut history, next, i);
    }
    next
}

fn main() {
    println!("{}", game(2020));
    println!("{}", game(30000000));
}
