const INPUT: &str = include_str!("../input/22.txt");

fn main() {
    let mut players = INPUT
        .split("\n\n")
        .map(|block| {
            let mut cards = block
                .lines()
                .skip(1)
                .map(|line| line.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            cards.reverse();
            cards
        })
        .collect::<Vec<_>>();
    while players.iter().all(|p| !p.is_empty()) {
        let card0 = players[0].pop().unwrap();
        let card1 = players[1].pop().unwrap();
        if card0 > card1 {
            players[0].insert(0, card0);
            players[0].insert(0, card1);
        } else {
            players[1].insert(0, card1);
            players[1].insert(0, card0);
        }
    }
    let score = players
        .iter()
        .find(|p| p.len() > 0)
        .map(|p| {
            p.iter()
                .enumerate()
                .fold(0, |score, (i, c)| score + ((i as u32 + 1) * c))
        })
        .unwrap();
    println!("{}", score);
}
