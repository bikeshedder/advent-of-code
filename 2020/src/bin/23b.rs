use std::ops::{Index, IndexMut};

use itertools::Itertools;

const INPUT: &str = "215694783";
const LEN: Cup = 1_000_000;
const MOVES: Cup = 10_000_000;

type Cup = u32;
struct Cups([Cup; LEN as usize]);

impl Cups {
    fn new() -> Self {
        Self([0; LEN as usize])
    }
    fn pick_up(&self, current_cup: Cup) -> [Cup; 3] {
        let c1 = self[current_cup];
        let c2 = self[c1];
        let c3 = self[c2];
        [c1, c2, c3]
    }
    fn put_down(&mut self, current_cup: Cup, picked_up: [Cup; 3]) {
        let mut prev_cup = current_cup;
        loop {
            prev_cup = prev_cup.checked_sub(1).unwrap_or(LEN as Cup-1);
            if !picked_up.contains(&prev_cup) {
                break;
            }
        }
        self[current_cup] = self[picked_up[2]];
        let tmp = self[prev_cup];
        self[prev_cup] = picked_up[0] as Cup;
        self[picked_up[2]] = tmp;
    }
}

impl Index<Cup> for Cups {
    type Output = Cup;
    fn index(&self, index: Cup) -> &Self::Output{
        &self.0[index as usize]
    }
}

impl IndexMut<Cup> for Cups {
    fn index_mut(&mut self, index: Cup) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

fn main() {
    let mut cups: Cups = Cups::new();
    let start = INPUT
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    for (c0, c1) in start.iter().tuple_windows() {
        cups[*c0 % LEN] = (*c1 % LEN) as Cup;
    }
    cups[0] = start[0];
    cups[start[start.len() - 1]] = start.len() as Cup + 1;
    for i in start.len() as Cup + 1..LEN {
        cups[i] = (i + 1) % LEN;
    }

    let mut current_cup = start[0];
    for _ in 1..=MOVES {
        let picked_up = cups.pick_up(current_cup);
        cups.put_down(current_cup, picked_up);
        current_cup = cups[current_cup];
    }

    println!("{}", cups[1] as u64 * cups[cups[1]] as u64);
}
