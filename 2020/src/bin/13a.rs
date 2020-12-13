const INPUT: &str = include_str!("../input/13.txt");

fn main() {
    let mut lines = INPUT.lines();
    let earliest: usize = lines.next().unwrap().parse().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|bus| match bus {
            "x" => None,
            n => n.parse::<usize>().ok(),
        });
    let bus = busses
        .map(|n| {
            let cycles = ((earliest - 1) / n) + 1;
            (cycles * n, n)
        })
        .min()
        .unwrap();
    let solution = bus.1 * (bus.0 - earliest);
    println!("{}", solution);
}
