use std::collections::HashSet;

const INPUT: &str = include_str!("../input/22.txt");

fn game(mut deck0: Vec<u32>, mut deck1: Vec<u32>) -> (bool, Vec<u32>, Vec<u32>) {
    let mut seen_decks = HashSet::<(Vec<u32>, Vec<u32>)>::new();
    let mut win0 = true;
    while !deck0.is_empty() && !deck1.is_empty() {
        if !seen_decks.insert((deck0.clone(), deck1.clone())) {
            win0 = true;
            break;
        }
        let card0 = deck0.pop().unwrap();
        let card1 = deck1.pop().unwrap();
        win0 = if card0 > deck0.len() as u32 || card1 > deck1.len() as u32 {
            card0 > card1
        } else {
            let result = game(
                deck0[deck0.len() - card0 as usize..].into(),
                deck1[deck1.len() - card1 as usize..].into(),
            );
            result.0
        };
        if win0 {
            deck0.insert(0, card0);
            deck0.insert(0, card1);
        } else {
            deck1.insert(0, card1);
            deck1.insert(0, card0);
        }
    }
    return (win0, deck0, deck1);
}

fn main() {
    let mut decks = INPUT.split("\n\n").map(|block| {
        let mut cards = block
            .lines()
            .skip(1)
            .map(|line| line.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        cards.reverse();
        cards
    });
    let result = game(decks.next().unwrap(), decks.next().unwrap());
    let winner = if result.0 { result.1 } else { result.2 };
    let score = winner
        .iter()
        .enumerate()
        .fold(0, |score, (i, c)| score + ((i as u32 + 1) * c));
    println!("{}", score);
}
