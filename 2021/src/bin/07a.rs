use std::collections::VecDeque;

const INPUT: &str = include_str!("../input/07.txt");

fn main() {
    let positions = INPUT
        .trim()
        .split(',')
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut submarines: VecDeque<(usize, usize)> = VecDeque::new();
    for position in positions.iter() {
        match submarines.binary_search_by(|p| p.0.cmp(position)) {
            Ok(idx) => submarines[idx].1 += 1,
            Err(idx) => submarines.insert(idx, (*position, 1)),
        }
    }
    let mut fuel = 0;
    while submarines.len() > 1 {
        if submarines.front().unwrap().1 <= submarines.back().unwrap().1 {
            let first = submarines.pop_front().unwrap();
            let front = submarines.front_mut().unwrap();
            front.1 += first.1;
            fuel += first.1 * (front.0 - first.0);
        } else {
            let last = submarines.pop_back().unwrap();
            let back = submarines.back_mut().unwrap();
            back.1 += last.1;
            fuel += last.1 * (last.0 - back.0);
        }
    }
    println!("{}", fuel);
}
