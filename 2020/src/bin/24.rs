use std::collections::HashSet;

use regex::Regex;

type Coord = (i32, i32);

const INPUT: &str = include_str!("../input/24.txt");

fn get_neighbors(c: Coord) -> [Coord; 6] {
    [
        (c.0 + 1, c.1),     // e
        (c.0, c.1 - 1),     // se
        (c.0 - 1, c.1 - 1), // sw
        (c.0 - 1, c.1),     // w
        (c.0, c.1 + 1),     // nw
        (c.0 + 1, c.1 + 1), // ne
    ]
}

fn get_neighbor(c: Coord, dir: &str) -> Coord {
    match dir {
        "e" => (c.0 + 1, c.1),
        "se" => (c.0, c.1 - 1),
        "sw" => (c.0 - 1, c.1 - 1),
        "w" => (c.0 - 1, c.1),
        "nw" => (c.0, c.1 + 1),
        "ne" => (c.0 + 1, c.1 + 1),
        _ => unreachable!(),
    }
}

fn follow_path(c: Coord, path: &[&str]) -> Coord {
    path.iter().fold(c, |c, dir| get_neighbor(c, dir))
}

#[derive(Clone, Default)]
struct Floor(HashSet<Coord>);

impl Floor {
    fn flip(&mut self, c: Coord) {
        if !self.0.remove(&c) {
            self.0.insert(c);
        }
    }
    fn flip_to_white(&mut self, c: Coord) {
        self.0.remove(&c);
    }
    fn flip_to_black(&mut self, c: Coord) {
        self.0.insert(c);
    }
    fn black_tiles(&self) -> usize {
        self.0.len()
    }
}

fn main() {
    let re = Regex::new("e|se|sw|w|nw|ne").unwrap();
    let paths = INPUT
        .lines()
        .map(|line| re.find_iter(line).map(|c| c.as_str()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // part 1
    let mut floor = Floor::default();
    for path in paths {
        let c = follow_path((0, 0), &path);
        floor.flip(c);
    }
    println!("{}", floor.black_tiles());
    // part 2
    for _ in 1..=100 {
        let black_tiles = floor.0.clone();
        let mut white_tiles = HashSet::<Coord>::new();
        for &c in black_tiles.iter() {
            let mut black_neighbors = 0;
            for nc in get_neighbors(c).iter() {
                if black_tiles.contains(nc) {
                    black_neighbors += 1;
                } else {
                    white_tiles.insert(*nc);
                }
            }
            if black_neighbors == 0 || black_neighbors > 2 {
                floor.flip_to_white(c);
            }
        }
        for &c in white_tiles.iter() {
            let mut black_neighbors = 0;
            for nc in get_neighbors(c).iter() {
                if black_tiles.contains(nc) {
                    black_neighbors += 1;
                }
            }
            if black_neighbors == 2 {
                floor.flip_to_black(c);
            }
        }
    }
    println!("{}", floor.black_tiles());
}
