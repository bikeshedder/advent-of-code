use std::cmp::{max, min};

const INPUT: &str = include_str!("../input/11.txt");

#[derive(Copy, Clone, PartialEq, Eq)]
enum Position {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

#[derive(Clone)]
struct Plan {
    data: Vec<Vec<Position>>,
    width: usize,
    height: usize,
}

impl Plan {
    fn parse(s: &str) -> Self {
        let data: Vec<Vec<Position>> = s
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| match c {
                        'L' => Position::EmptySeat,
                        '.' => Position::Floor,
                        _ => panic!("unexpected character: {}", c),
                    })
                    .collect()
            })
            .collect();
        Self {
            width: data.first().unwrap().len(),
            height: data.len(),
            data,
        }
    }
    fn occupied_seats(&self) -> usize {
        let mut count = 0;
        for row in self.data.iter() {
            for pos in row.iter() {
                if *pos == Position::OccupiedSeat {
                    count += 1;
                }
            }
        }
        count
    }
    fn ascii_art(&self) -> String {
        self.data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pos| match pos {
                        Position::EmptySeat => 'L',
                        Position::OccupiedSeat => '#',
                        Position::Floor => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl std::fmt::Display for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ascii_art())
    }
}

struct Simulation {
    plan: Plan,
}

impl Simulation {
    fn new(plan: Plan) -> Self {
        Self { plan }
    }
    fn step(&mut self) -> bool {
        let mut changed = false;
        let old_plan = self.plan.clone();
        for y in 0..self.plan.height {
            let y0 = max(y as isize - 1, 0) as usize;
            let y1 = min(y + 1, self.plan.height - 1);
            for x in 0..self.plan.width {
                let x0 = max(x as isize - 1, 0) as usize;
                let x1 = min(x + 1, self.plan.width - 1);
                let mut occupied_seats = 0;
                for y in y0..=y1 {
                    for x in x0..=x1 {
                        if old_plan.data[y][x] == Position::OccupiedSeat {
                            occupied_seats += 1;
                        }
                    }
                }
                match old_plan.data[y][x] {
                    Position::EmptySeat if occupied_seats == 0 => {
                        self.plan.data[y][x] = Position::OccupiedSeat;
                        changed = true;
                    }
                    Position::OccupiedSeat if occupied_seats > 4 => {
                        self.plan.data[y][x] = Position::EmptySeat;
                        changed = true;
                    }
                    _ => {}
                }
            }
        }
        changed
    }
}

fn main() {
    let mut sim = Simulation::new(Plan::parse(INPUT));
    while sim.step() {}
    println!("{}", sim.plan.occupied_seats());
}
