use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("../input/20.txt");

#[derive(Debug, Default)]
struct Tile {
    id: u16,
    data: [u16; 10],
    borders: [u16; 8],
}

fn vertical(data: &[u16; 10], col: usize) -> u16 {
    data.iter()
        .fold(0u16, |border, line| border << 1 | ((line >> col) & 1))
}

fn rev_border(border: u16) -> u16 {
    (0..10).fold(0u16, |r, i| r << 1 | ((border >> i) & 1))
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines.next().unwrap()[5..]
            .split(":")
            .next()
            .unwrap()
            .parse::<u16>()
            .unwrap();
        let data = lines
            .map(|line| {
                line.chars().fold(0u16, |data, c| {
                    data << 1
                        | match c {
                            '#' => 1,
                            _ => 0,
                        }
                })
            })
            .enumerate()
            .fold([0u16; 10], |mut arr, (i, data)| {
                arr[i] = data;
                arr
            });
        let borders = [
            // normal borders
            data[0],
            vertical(&data, 0),
            rev_border(data[9]),
            rev_border(vertical(&data, 9)),
            // flipped borders
            rev_border(data[0]),
            rev_border(vertical(&data, 0)),
            data[9],
            vertical(&data, 9),
        ];
        Ok(Self { id, data, borders })
    }
}

fn main() {
    let tiles: Vec<Tile> = INPUT
        .split("\n\n")
        .map(|block| block.parse::<Tile>().unwrap())
        .collect();
    let mut borders: HashMap<u16, Vec<u16>> = HashMap::new();
    for tile in tiles {
        for border in tile.borders.iter() {
            borders
                .entry(*border)
                .and_modify(|t| t.push(tile.id))
                .or_insert_with(|| vec![tile.id]);
        }
    }
    let mut found: HashMap<u16, usize> = HashMap::new();
    for (_, tile_ids) in borders.iter() {
        if tile_ids.len() == 2 {
            for tile_id in tile_ids {
                found
                    .entry(*tile_id)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }
    let solution = found
        .iter()
        .filter(|(_, count)| **count == 4)
        .map(|(tile_id, _)| *tile_id as u64)
        .product::<u64>();
    println!("{}", solution);
}
