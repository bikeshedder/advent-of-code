const INPUT: &str = include_str!("../input/01.txt");

fn main() {
    let output = INPUT
        .trim()
        .chars()
        .enumerate()
        .scan(0, |floor, (i, c)| {
            *floor += match c {
                '(' => 1,
                ')' => -1,
                _ => panic!(),
            };
            Some((i, *floor))
        })
        .find_map(|(i, floor)| if floor == -1 { Some(i) } else { None })
        .unwrap();
    println!("{}", output + 1);
}
