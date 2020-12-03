const INPUT: &[u8] = include_bytes!("2020-03.txt");
const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn main() {
    let solution = SLOPES
        .iter()
        .map(|(right, down)| {
            INPUT
                .split(|&c| c == b'\n')
                .filter(|line| !line.is_empty())
                .enumerate()
                .filter_map(|(i, line)| match i % down {
                    0 => Some(line),
                    _ => None,
                })
                .fold((0, 0), |(idx, cnt), line| {
                    (
                        idx + right,
                        cnt + match line[idx % line.len()] {
                            b'.' => 0,
                            b'#' => 1,
                            _ => unreachable!(),
                        },
                    )
                })
                .1
        })
        .fold(1, |prod, cnt| prod * cnt);
    println!("{}", solution);
}
