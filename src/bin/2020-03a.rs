const INPUT: &[u8] = include_bytes!("2020-03.txt");
const STEP: usize = 3;

fn main() {
    let solution = INPUT
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .fold((0, 0), |(idx, cnt), line| {
            (
                idx + STEP,
                cnt + match line[idx % line.len()] {
                    b'.' => 0,
                    b'#' => 1,
                    _ => unreachable!(),
                },
            )
        })
        .1;
    println!("{}", solution);
}
