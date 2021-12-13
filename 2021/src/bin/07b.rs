use std::collections::VecDeque;

const INPUT: &str = include_str!("../input/07.txt");

fn fuel_cost(distance: i32) -> i32 {
    let distance = distance.abs();
    ((distance + 1) * distance) / 2
}

fn main() {
    let positions = INPUT
        .trim()
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut submarines: VecDeque<(i32, i32)> = VecDeque::new();
    for position in positions.iter() {
        match submarines.binary_search_by(|p| p.0.cmp(position)) {
            Ok(idx) => submarines[idx].1 += 1,
            Err(idx) => submarines.insert(idx, (*position, 1)),
        }
    }
    let target_positions = submarines.front().unwrap().0..submarines.back().unwrap().0;
    let answer: i32 = target_positions
        .map(|to_pos| {
            submarines
                .iter()
                .map(|(from_pos, cnt)| cnt * fuel_cost(from_pos - to_pos))
                .sum()
        })
        .min()
        .unwrap();
    println!("{}", answer);
}
