use std::char;

const INPUT: &str = "215694783";
const LEN: usize = 1_000_000;
const MOVES: usize = 10_000_000;

fn main() {
    let mut cups: Vec<i64> = INPUT
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();
    cups.reserve(LEN - cups.len());
    for c in INPUT.len() + 1..=LEN {
        cups.push(c as i64);
    }
    let start_time = std::time::SystemTime::now();
    for m in 1..=MOVES {
        let insert_idx = (4..LEN)
            .min_by_key(|i| (cups[0] - cups[*i]).rem_euclid(LEN as i64))
            .unwrap();
        cups[1..insert_idx + 1].rotate_left(3);
        cups.rotate_left(1);
        if m % 1000 == 0 {
            let elapsed = std::time::SystemTime::now().duration_since(start_time).unwrap();
            let remaining = (elapsed / m as u32) * (MOVES - m) as u32;
            println!("move {} ({:.2}%) {:?}", m, m as f64 / MOVES as f64 * 100.0, remaining);
        }
    }
    let cup1_idx = cups.iter().position(|cup| *cup == 1).unwrap();
    cups.rotate_left(cup1_idx as usize);
    println!("{:?}", cups[0..10].iter().collect::<Vec<_>>());
    println!("{}", cups[1] * cups[2]);
}
