use std::{collections::HashMap, fmt::Display, str::FromStr};

use itertools::iproduct;

const INPUT: &str = include_str!("../input/20.txt");

const MAP_TILES: usize = 12;
const MAP_SIZE: usize = MAP_TILES * 8;

const MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];
const MONSTER_WAVES: usize = 15;
const MONSTER_HEIGHT: usize = MONSTER.len();
const MONSTER_WIDTH: usize = MONSTER[0].len();

#[derive(Debug, Default)]
struct Tile {
    id: u16,
    data: [u16; 10],
    borders: [u16; 8],
    neighbors: [Option<(usize, u16)>; 8],
    transform: usize,
}

fn flip_borders<T: Default + Copy>(data: &mut [T; 8]) {
    let mut new_data = [
        data[4], data[5], data[6], data[7], data[0], data[1], data[2], data[3],
    ];
    std::mem::swap(data, &mut new_data);
}

fn rotate_borders<T: Default + Copy>(data: &mut [T; 8]) {
    let mut new_data = [
        data[3], data[0], data[1], data[2], data[7], data[4], data[5], data[6],
    ];
    std::mem::swap(data, &mut new_data);
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .data
            .iter()
            .map(|row| {
                (0..10)
                    .map(|i| match (row >> (9 - i)) & 1 {
                        0 => '.',
                        1 => '#',
                        _ => unreachable!(),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", s)
    }
}

impl Tile {
    // swap X and Y axis
    fn flip(&mut self) {
        let mut new_data = [0u16; 10];
        for y in 0..10 {
            for x in 0..10 {
                new_data[x] |= ((self.data[y] >> x) & 1) << y;
            }
        }
        std::mem::swap(&mut new_data, &mut self.data);
        flip_borders(&mut self.borders);
        flip_borders(&mut self.neighbors);
        self.transform ^= 4;
    }
    // rotate
    fn rotate(&mut self, steps: usize) {
        for _ in 0..(steps % 4) {
            let mut new_data = [0u16; 10];
            for y in 0..10 {
                for x in 0..10 {
                    new_data[x] |= ((self.data[y] >> x) & 1) << (9 - y);
                }
            }
            std::mem::swap(&mut new_data, &mut self.data);
            rotate_borders(&mut self.borders);
            rotate_borders(&mut self.neighbors);
        }
        self.transform = (self.transform & !3) | (self.transform + steps) & 3;
    }
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
            rev_border(vertical(&data, 9)),
            rev_border(data[9]),
            vertical(&data, 0),
            // flipped borders
            rev_border(vertical(&data, 0)),
            data[9],
            vertical(&data, 9),
            rev_border(data[0]),
        ];
        let neighbors = [None; 8];
        Ok(Self {
            id,
            data,
            borders,
            neighbors,
            transform: 0,
        })
    }
}

fn prepare_map() -> [u128; MAP_SIZE] {
    let mut tiles: HashMap<u16, Tile> = INPUT
        .split("\n\n")
        .map(|block| block.parse::<Tile>().unwrap())
        .map(|tile| (tile.id, tile))
        .collect();
    let mut border_map: HashMap<u16, Vec<(u16, usize)>> = HashMap::new();
    for tile in tiles.values() {
        for (direction, border) in tile.borders.iter().enumerate() {
            border_map
                .entry(*border)
                .or_default()
                .push((tile.id, direction));
        }
    }
    for tiles_w_directions in border_map.values() {
        assert!(tiles_w_directions.len() <= 2);
        if tiles_w_directions.len() != 2 {
            continue;
        }
        let (tile0_id, dir0) = tiles_w_directions[0];
        let (tile1_id, dir1) = tiles_w_directions[1];
        let mut tile0 = tiles.get_mut(&tile0_id).unwrap();
        tile0.neighbors[dir0] = Some((dir1, tile1_id));
        let mut tile1 = tiles.get_mut(&tile1_id).unwrap();
        tile1.neighbors[dir1] = Some((dir0, tile0_id));
    }
    let mut map = [[0u16; MAP_TILES]; MAP_TILES];
    {
        let mut corners = tiles
            .values_mut()
            .filter(|tile| tile.neighbors.iter().filter(|n| n.is_some()).count() == 4)
            .collect::<Vec<_>>();
        assert_eq!(corners.len(), 4);
        let corner = &mut corners[0];
        // rotate first corner so that it has neighbors on the right (E)
        // and bottom (S).
        while corner.neighbors[1].is_none() || corner.neighbors[2].is_none() {
            corner.rotate(1);
        }
        map[0][0] = corner.id;
    }
    // fill first row
    for x in 0..MAP_TILES - 1 {
        let tile = &tiles[&map[0][x]];
        let (neighbor_transform, neighbor_id) = tile.neighbors[1].unwrap();
        let neighbor = tiles.get_mut(&neighbor_id).unwrap();
        map[0][x + 1] = neighbor_id;
        if neighbor_transform & 4 == 0 {
            neighbor.flip();
            neighbor.rotate(neighbor_transform % 4);
        } else {
            neighbor.rotate(neighbor_transform % 4);
        }
    }
    // fill other rows
    for y in 0..MAP_TILES - 1 {
        for x in 0..MAP_TILES {
            let tile = &tiles[&map[y][x]];
            let (neighbor_transform, neighbor_id) = tile.neighbors[2].unwrap();
            let neighbor = tiles.get_mut(&neighbor_id).unwrap();
            map[y + 1][x] = neighbor_id;
            if neighbor_transform & 4 == 0 {
                neighbor.flip();
                neighbor.rotate(neighbor_transform % 4 + 1);
            } else {
                neighbor.rotate(neighbor_transform % 4 + 1);
            }
        }
    }

    let mut data = [0u128; MAP_SIZE];
    for y in 0..MAP_TILES {
        for x in (0..MAP_TILES).rev() {
            let tile = tiles.get(&map[y][x]).unwrap();
            for row in 0..8 {
                data[y * 8 + row] <<= 8;
                data[y * 8 + row] |= ((tile.data[row + 1] as u128) >> 1) & 255;
            }
        }
    }

    data
}

fn flip_map(data: &[u128; MAP_SIZE]) -> [u128; MAP_SIZE] {
    let mut new_data = [0u128; MAP_SIZE];
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            new_data[x] |= ((data[y] >> x) & 1) << y;
        }
    }
    new_data
}

// rotate
fn rotate_map(data: &[u128; MAP_SIZE]) -> [u128; MAP_SIZE] {
    let mut new_data = [0u128; MAP_SIZE];
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            new_data[x] |= ((data[y] >> x) & 1) << (MAP_SIZE - y - 1);
        }
    }
    new_data
}

fn prepare_monster() -> [u128; MONSTER_HEIGHT] {
    let mut monster = [0u128; MONSTER_HEIGHT];
    for y in 0..MONSTER_HEIGHT {
        let row = &mut monster[y];
        for x in 0..MONSTER[y].len() {
            *row <<= 1;
            *row |= match MONSTER[y].chars().nth(x).unwrap() {
                '#' => 1,
                _ => 0,
            }
        }
    }
    monster
}

fn count_monsters(map: &[u128; MAP_SIZE], monster: &[u128; MONSTER_HEIGHT]) -> usize {
    iproduct!(
        0..(MAP_SIZE - MONSTER_HEIGHT + 1),
        0..(MAP_SIZE - MONSTER_WIDTH + 1)
    )
    .filter(|(y, x)| {
        (0..MONSTER_HEIGHT).all(|row| (map[y + row] >> x) & monster[row] == monster[row])
    })
    .count()
}

fn map_to_string(map: &[u128; MAP_SIZE]) -> String {
    map.iter()
        .map(|row| {
            (0..MAP_SIZE)
                .map(|i| match (row >> (MAP_SIZE - i - 1)) & 1 {
                    0 => '.',
                    1 => '#',
                    _ => unreachable!(),
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let map_0 = prepare_map();
    let map_1 = rotate_map(&map_0);
    let map_2 = rotate_map(&map_1);
    let map_3 = rotate_map(&map_2);
    let map_4 = flip_map(&map_0);
    let map_5 = rotate_map(&map_4);
    let map_6 = rotate_map(&map_5);
    let map_7 = rotate_map(&map_6);
    let monster = prepare_monster();
    let monster_count = count_monsters(&map_0, &monster)
        + count_monsters(&map_1, &monster)
        + count_monsters(&map_2, &monster)
        + count_monsters(&map_3, &monster)
        + count_monsters(&map_4, &monster)
        + count_monsters(&map_5, &monster)
        + count_monsters(&map_6, &monster)
        + count_monsters(&map_7, &monster);
    let map_waves = map_0.iter().map(|row| {
        (0..MAP_SIZE).filter(|i| (row >> i) & 1 == 1).count()
    }).sum::<usize>();
    let solution = map_waves - monster_count * MONSTER_WAVES;
    println!("{}", solution);
}
