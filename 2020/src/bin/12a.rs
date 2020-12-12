const INPUT: &str = include_str!("../input/12.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point(isize, isize);

impl Point {
    fn rotate(&self, x: isize) -> Self {
        match (x + 360) % 360 {
            0 => *self,
            90 => Self(self.1, -self.0),
            180 => Self(-self.0, -self.1),
            270 => Self(-self.1, self.0),
            _ => panic!("Unsupported rotation: {}", x),
        }
    }
    fn rotate_assign(&mut self, x: isize) {
        let mut y = self.rotate(x);
        std::mem::swap(self, &mut y);
    }
    fn manhattan_distance(&self) -> usize {
        (self.0.abs() + self.1.abs()) as usize
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl std::ops::Mul<isize> for Point {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(Debug)]
enum Instruction {
    Move(Point),
    Rotate(isize),
    Forward(isize),
}

impl std::str::FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s[1..].parse::<isize>()?;
        Ok(match &s[0..1] {
            "N" => Instruction::Move(Point(0, x)),
            "E" => Instruction::Move(Point(x, 0)),
            "S" => Instruction::Move(Point(0, -x)),
            "W" => Instruction::Move(Point(-x, 0)),
            "L" => Instruction::Rotate(-x),
            "R" => Instruction::Rotate(x),
            "F" => Instruction::Forward(x),
            _ => panic!("Unsupported instruction: {}", s),
        })
    }
}

#[derive(Debug)]
struct Boat {
    pos: Point,
    dir: Point,
}

impl Boat {
    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(x) => self.pos += *x,
            Instruction::Forward(x) => self.pos += self.dir * (*x),
            Instruction::Rotate(x) => self.dir.rotate_assign(*x),
        }
    }
}

fn main() {
    let instructions = INPUT
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    let mut boat = Boat {
        pos: Point(0, 0),
        dir: Point(1, 0),
    };
    for instruction in instructions {
        boat.execute(&instruction);
    }
    println!("{}", boat.pos.manhattan_distance());
}
