const INPUT: &str = include_str!("2015-01.txt");

fn main() {
    let mut floor = 0;
    for (i, c) in INPUT.trim().chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!(),
        }
        if floor == -1 {
            println!("{}", i + 1);
            return;
        }
    }
}
