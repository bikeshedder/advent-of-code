use std::{collections::HashSet, hash::Hash, ops::Add};

use regex::Regex;

const INPUT: &str = include_str!("../input/24.txt");
const NEIGHBORS: [Coord; 6] = [
    Coord(1, 0),   // e
    Coord(0, -1),  // se
    Coord(-1, -1), // sw
    Coord(-1, 0),  // w
    Coord(0, 1),   // nw
    Coord(1, 1),   // ne
];

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Coord(i32, i32);

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Coord {
    fn from_dir(s: &str) -> Coord {
        match s {
            "e" => NEIGHBORS[0],
            "se" => NEIGHBORS[1],
            "sw" => NEIGHBORS[2],
            "w" => NEIGHBORS[3],
            "nw" => NEIGHBORS[4],
            "ne" => NEIGHBORS[5],
            _ => unreachable!(),
        }
    }
    fn follow_path(&self, path: &[Coord]) -> Coord {
        path.iter().fold(*self, |c, dir| c + *dir)
    }
}

type Floor = HashSet<Coord>;

fn main() {
    let re = Regex::new("e|se|sw|w|nw|ne").unwrap();
    let paths = INPUT
        .lines()
        .map(|line| {
            re.find_iter(line)
                .map(|c| Coord::from_dir(c.as_str()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // part 1
    let mut floor = Floor::new();
    let center = Coord(0, 0);
    for path in paths {
        let c = center.follow_path(&path);
        if !floor.remove(&c) {
            floor.insert(c);
        }
    }
    println!("{}", floor.len());
    // part 2
    for _ in 1..=100 {
        let black_tiles = floor.clone();
        let mut white_tiles = Floor::new();
        for &c in black_tiles.iter() {
            let mut black_neighbors = 0;
            for &n in NEIGHBORS.iter() {
                let nc = c + n;
                if black_tiles.contains(&nc) {
                    black_neighbors += 1;
                } else {
                    white_tiles.insert(nc);
                }
            }
            if black_neighbors == 0 || black_neighbors > 2 {
                floor.remove(&c);
            }
        }
        for &c in white_tiles.iter() {
            let mut black_neighbors = 0;
            for &n in NEIGHBORS.iter() {
                let nc = c + n;
                if black_tiles.contains(&nc) {
                    black_neighbors += 1;
                }
            }
            if black_neighbors == 2 {
                floor.insert(c);
            }
        }
    }
    println!("{}", floor.len());
}
