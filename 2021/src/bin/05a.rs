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
        if self.0.x == self.1.x {
            // vertical
            let x = self.0.x;
            let y0 = self.0.y.min(self.1.y);
            let y1 = self.0.y.max(self.1.y);
            for y in y0..=y1 {
                map.0[y][x] += 1;
            }
        }
        if self.0.y == self.1.y {
            // horizontal
            let y = self.0.y;
            let x0 = self.0.x.min(self.1.x);
            let x1 = self.0.x.max(self.1.x);
            for x in x0..=x1 {
                map.0[y][x] += 1;
            }
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
