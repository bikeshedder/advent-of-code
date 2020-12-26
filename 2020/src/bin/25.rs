const INPUT: &str = include_str!("../input/25.txt");

const SUBJECT_NUMBER: u64 = 7;
const DIV: u64 = 20201227;

fn main() {
    let mut pubkeys = INPUT.lines().map(|line| line.parse::<u64>().unwrap());
    let pubkey_card = pubkeys.next().unwrap();
    let pubkey_door = pubkeys.next().unwrap();
    let loopsize_card = (1u64..)
        .scan(1, |key, loop_size| {
            *key *= SUBJECT_NUMBER;
            *key %= DIV;
            Some((*key, loop_size))
        })
        .find(|(k, _)| *k == pubkey_card)
        .unwrap()
        .1;
    let answer = (1..loopsize_card).fold(pubkey_door, |k, _| (k * pubkey_door) % DIV);
    println!("{}", answer);
}
