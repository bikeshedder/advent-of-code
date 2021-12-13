const INPUT: &str = include_str!("../input/05.txt");
const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn parse(s: &str) -> Point {
        let mut it = s.split(',');
        Self {
            x: it.next().unwrap().parse().unwrap(),
            y: it.next().unwrap().parse().unwrap(),
        }
    }
}

struct Line(Point, Point);

impl Line {
    pub fn parse(s: &str) -> Self {
        let mut it = s.trim().split(" -> ");
        Self(
            Point::parse(it.next().unwrap()),
            Point::parse(it.next().unwrap()),
        )
    }
    pub fn mark(&self, map: &mut Map) {
        let x_diff = self.1.x as isize - self.0.x as isize;
        let y_diff = self.1.y as isize - self.0.y as isize;
        let steps = x_diff.abs().max(y_diff.abs());
        let dx = x_diff / steps;
        let dy = y_diff / steps;
        for i in 0..=steps {
            let x = self.0.x as isize + i * dx;
            let y = self.0.y as isize + i * dy;
            map.0[y as usize][x as usize] += 1;
        }
    }
}

struct Map([[u16; WIDTH]; HEIGHT]);

impl Map {
    pub fn dangerous_coords(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|c| **c > 1).count())
            .sum::<usize>()
    }
}

impl Default for Map {
    fn default() -> Self {
        let data = [[0; WIDTH]; HEIGHT];
        Self(data)
    }
}

fn main() {
    let lines = INPUT.trim().lines().map(Line::parse).collect::<Vec<_>>();
    let mut map = Map::default();
    for line in lines {
        line.mark(&mut map);
    }
    let answer = map.dangerous_coords();
    println!("{}", answer);
}
