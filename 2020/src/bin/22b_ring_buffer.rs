use std::{cmp::min, collections::HashSet, hash::Hash, str::FromStr};

const INPUT: &str = include_str!("../input/22.txt");
const MAX_DECK_SIZE: usize = 50;

type Card = u8;

#[derive(Clone, Copy, Debug, Eq)]
struct Deck {
    cards: [Card; MAX_DECK_SIZE],
    index: usize,
    size: usize,
}

impl Deck {
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    fn draw(&mut self) -> Card {
        self.size -= 1;
        let card = self.cards[(self.index + self.size) % MAX_DECK_SIZE];
        card
    }
    fn size(&self) -> usize {
        self.size
    }
    fn copy(&self, card: Card) -> Self {
        let copy = Self {
            cards: self.cards.clone(),
            index: (MAX_DECK_SIZE + self.index + self.size - card as usize)
                % MAX_DECK_SIZE as usize,
            size: card as usize,
        };
        copy
    }
    fn add(&mut self, card: Card) {
        self.index = (self.index + MAX_DECK_SIZE - 1) % MAX_DECK_SIZE;
        self.cards[self.index] = card;
        self.size += 1;
    }
    fn score(&self) -> usize {
        self.cards()
            .iter()
            .enumerate()
            .fold(0, |score, (i, c)| score + (i + 1) * (*c) as usize)
    }
    fn cards(&self) -> Vec<Card> {
        let mut cards = Vec::with_capacity(self.size);
        for i in self.index..min(self.index + self.size, MAX_DECK_SIZE) {
            cards.push(self.cards[i]);
        }
        if self.index + self.size >= MAX_DECK_SIZE {
            for i in 0..((self.index + self.size) % MAX_DECK_SIZE) {
                cards.push(self.cards[i]);
            }
        }
        cards
    }
    fn get(&self, index: usize) -> Card {
        self.cards[(self.index + index) % MAX_DECK_SIZE]
    }
}

impl Hash for Deck {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.cards[self.index..min(self.index + self.size, MAX_DECK_SIZE)]);
        if self.index + self.size >= MAX_DECK_SIZE {
            state.write(&self.cards[0..((self.index + self.size) % MAX_DECK_SIZE)]);
        }
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && (0..self.size).all(|i| self.get(i) == other.get(i))
    }
}

impl FromStr for Deck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = s
            .lines()
            .skip(1)
            .map(|line| line.parse::<Card>().unwrap())
            .collect::<Vec<_>>();
        cards.reverse();
        let mut cards_array = [Card::default(); 50];
        for (i, card) in cards.iter().enumerate() {
            cards_array[i] = *card;
        }
        Ok(Self {
            cards: cards_array,
            index: 0,
            size: cards.len(),
        })
    }
}

fn game(mut deck0: Deck, mut deck1: Deck) -> (bool, Deck, Deck) {
    let mut seen_decks = HashSet::<(Deck, Deck)>::new();
    let mut win0 = true;
    while !deck0.is_empty() && !deck1.is_empty() {
        if !seen_decks.insert((deck0, deck1)) {
            win0 = true;
            break;
        }
        let card0 = deck0.draw();
        let card1 = deck1.draw();
        win0 = if card0 as usize > deck0.size() || card1 as usize > deck1.size() {
            card0 > card1
        } else {
            let result = game(deck0.copy(card0), deck1.copy(card1));
            result.0
        };
        if win0 {
            deck0.add(card0);
            deck0.add(card1);
        } else {
            deck1.add(card1);
            deck1.add(card0);
        }
    }
    return (win0, deck0, deck1);
}

fn main() {
    let mut decks = INPUT
        .split("\n\n")
        .map(|block| block.parse::<Deck>().unwrap());
    let result = game(decks.next().unwrap(), decks.next().unwrap());
    let winner = if result.0 { result.1 } else { result.2 };
    println!("{}", winner.score());
}
