const INPUT: &str = include_str!("../input/04.txt");

struct Board(Vec<(i32, bool)>, Option<usize>);

impl Board {
    pub fn parse(s: &str) -> Self {
        Self(
            s.split_whitespace()
                .map(|n| (n.parse::<i32>().unwrap(), false))
                .collect(),
            None,
        )
    }
    pub fn mark(&mut self, n: i32) {
        for (number, mark) in self.0.iter_mut() {
            if n == *number {
                *mark = true;
            }
        }
    }
    pub fn winner(&self) -> bool {
        (0..5)
            .map(|i| {
                // row
                self.0[i * 5..i * 5 + 5].iter().all(|&(_, mark)| mark)
                ||
                // column
                self.0.iter().skip(i).step_by(5).all(|&(_, mark)| mark)
            })
            .any(|b| b)
    }
    pub fn unmarked_sum(&self) -> i32 {
        self.0
            .iter()
            .filter_map(|(number, mark)| if !mark { Some(number) } else { None })
            .sum()
    }
}

fn play<'a>(numbers: &[i32], boards: &'a mut [Board]) -> i32 {
    let mut last_score = 0;
    for (round, &number) in numbers.iter().enumerate() {
        for board in boards.iter_mut() {
            if board.1.is_none() {
                board.mark(number);
                if board.winner() {
                    board.1 = Some(round);
                    last_score = board.unmarked_sum() * number;
                }
            }
        }
    }
    last_score
}

fn main() {
    let mut blocks = INPUT.trim().split("\n\n");
    let numbers: Vec<i32> = blocks
        .next()
        .unwrap()
        .split(",")
        .map(|s| str::parse(s).unwrap())
        .collect();
    let mut boards: Vec<Board> = blocks.map(Board::parse).collect();
    let answer = play(&numbers, &mut boards);
    println!("{}", answer);
}
