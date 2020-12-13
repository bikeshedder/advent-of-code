const INPUT: &str = include_str!("../input/13.txt");

fn main() {
    let solution = INPUT
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|bus| bus.parse::<usize>().unwrap_or(1))
        .enumerate()
        .fold((0, 1), |(mut t, step_by), (n, bus)| {
            while (t + n) % bus != 0 {
                t += step_by;
            }
            (t, step_by * bus)
        })
        .0;
    println!("{}", solution);
}
