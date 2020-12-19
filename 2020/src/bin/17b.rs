use std::fmt::Display;

const INPUT: &str = include_str!("../input/17.txt");
const STEPS: usize = 6;
const SIZE: isize = 12;

type Point = (isize, isize, isize, isize);

#[derive(Clone, Debug)]
struct Space {
    data: Vec<bool>,
    size: Point,
}

impl Space {
    fn new(size: Point) -> Self {
        Self {
            data: vec![false; (size.0 * size.1 * size.2 * size.3) as usize],
            size,
        }
    }
    fn calc_index(&self, p: Point) -> usize {
        (self.size.0 * self.size.1 * self.size.2 * p.0
            + self.size.0 * self.size.1 * p.1
            + self.size.0 * p.2
            + p.3) as usize
    }
    fn active(&self) -> usize {
        self.data.iter().map(|b| *b as usize).sum::<usize>()
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .data
            .iter()
            .enumerate()
            .map(|(i, b)| {
                match (
                    i > 0 && i % (self.size.0 * self.size.1) as usize == 0,
                    i > 0 && i % self.size.1 as usize == 0,
                    b,
                ) {
                    (true, _, false) => "\n\n.",
                    (true, _, true) => "\n\n#",
                    (_, true, false) => "\n.",
                    (_, true, true) => "\n#",
                    (false, false, false) => ".",
                    (false, false, true) => "#",
                }
            })
            .collect::<String>();
        write!(f, "{}", s)
    }
}

impl std::ops::Index<Point> for Space {
    type Output = bool;
    fn index(&self, p: Point) -> &Self::Output {
        let index = self.calc_index(p);
        &self.data[index]
    }
}

impl std::ops::IndexMut<Point> for Space {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        let index = self.calc_index(p);
        &mut self.data[index]
    }
}

fn active_region(space: &Space, p: Point) -> usize {
    (-1..=1)
        .map(|w| {
            (-1..=1)
                .map(|z| {
                    (-1..=1)
                        .map(|y| {
                            (-1..=1)
                                .map(|x| space[(p.0 + w, p.1 + z, p.2 + y, p.3 + x)] as usize)
                                .sum::<usize>()
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn step(space: &mut Space) {
    let clone = space.clone();
    for w in 1..space.size.2 - 1 {
        for z in 1..space.size.2 - 1 {
            for y in 1..space.size.1 - 1 {
                for x in 1..space.size.0 - 1 {
                    let p = (w, z, y, x);
                    space[p] = match (clone[p], active_region(&clone, p)) {
                        (true, 3) | (true, 4) => true,
                        (false, 3) => true,
                        _ => false,
                    }
                }
            }
        }
    }
}

fn main() {
    let size = (SIZE * 2 + 1, SIZE * 2 + 1, SIZE * 2 + 1, SIZE * 2 + 1);
    let mut space = Space::new(size);
    let input: Vec<Vec<bool>> = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let input_width = input.first().unwrap().len() as isize;
    let input_height = input.len() as isize;
    let input_origin = (
        -input_height / 2 as isize + SIZE,
        -input_width / 2 as isize + SIZE,
    );
    for y in 0..input_height {
        for x in 0..input_width {
            space[(SIZE, SIZE, y + input_origin.0, x + input_origin.1)] =
                input[y as usize][x as usize];
        }
    }
    for _ in 0..STEPS {
        step(&mut space);
    }
    println!("{}", space.active());
}
