use std::collections::HashSet;

const INPUT: &str = include_str!("2015-03.txt");

fn main() {
    let output = INPUT
        .trim()
        .chars()
        .enumerate()
        .scan([(0, 0), (0, 0)], |t, (i, c)| {
            let xy = &mut t[i % 2];
            *xy = match c {
                '>' => (xy.0 + 1, xy.1),
                '<' => (xy.0 - 1, xy.1),
                '^' => (xy.0, xy.1 + 1),
                'v' => (xy.0, xy.1 - 1),
                _ => panic!(),
            };
            Some(*xy)
        })
        .fold(
            [(0, 0)]
                .iter()
                .map(|&xy| xy.clone())
                .collect::<HashSet<(i64, i64)>>(),
            |mut set, xy| {
                set.insert(xy);
                set
            },
        )
        .len();
    println!("{:?}", output);
}
