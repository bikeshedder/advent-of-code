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
    fn can_see_occupied_seat_dir(&self, pos: (usize, usize), dir: (isize, isize)) -> bool {
        let mut pos = (pos.0 as isize, pos.1 as isize);
        loop {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            if pos.0 < 0 || pos.0 as usize >= self.height {
                break;
            };
            if pos.1 < 0 || pos.1 as usize >= self.width {
                break;
            };
            match self.data[pos.0 as usize][pos.1 as usize] {
                Position::EmptySeat => return false,
                Position::OccupiedSeat => return true,
                _ => {}
            }
        }
        false
    }
    fn can_see_occupied_seats(&self, pos: (usize, usize)) -> usize {
        self.can_see_occupied_seat_dir(pos, (-1, -1)) as usize
            + self.can_see_occupied_seat_dir(pos, (-1, 0)) as usize
            + self.can_see_occupied_seat_dir(pos, (-1, 1)) as usize
            + self.can_see_occupied_seat_dir(pos, (0, -1)) as usize
            + self.can_see_occupied_seat_dir(pos, (0, 1)) as usize
            + self.can_see_occupied_seat_dir(pos, (1, -1)) as usize
            + self.can_see_occupied_seat_dir(pos, (1, 0)) as usize
            + self.can_see_occupied_seat_dir(pos, (1, 1)) as usize
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
            for x in 0..self.plan.width {
                let occupied_seats = old_plan.can_see_occupied_seats((y, x));
                match old_plan.data[y][x] {
                    Position::EmptySeat if occupied_seats == 0 => {
                        self.plan.data[y][x] = Position::OccupiedSeat;
                        changed = true;
                    }
                    Position::OccupiedSeat if occupied_seats >= 5 => {
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
