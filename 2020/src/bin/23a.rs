use std::char;

const INPUT: &str = "215694783";
const LEN: usize = INPUT.len();
const MOVES: usize = 100;

fn main() {
    // parse input
    let mut cups: Vec<i8> = INPUT
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect();
    // play the game
    for _ in 0..MOVES {
        let insert_idx = (4..LEN)
            .min_by_key(|i| (cups[0] - cups[*i]).rem_euclid(LEN as i8))
            .unwrap();
        cups[1..insert_idx + 1].rotate_left(3);
        cups.rotate_left(1);
    }
    // move cup 1 to the front
    let cup1_idx = cups.iter().position(|cup| *cup == 1).unwrap();
    cups.rotate_left(cup1_idx as usize);
    // output answer
    let answer = cups[1..]
        .iter()
        .map(|d| char::from_digit(*d as u32, 10).unwrap())
        .collect::<String>();
    println!("{}", answer);
}
