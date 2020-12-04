const INPUT: &[u8] = include_bytes!("../input/03.txt");
const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn main() {
    let solution = SLOPES
        .iter()
        .map(|(right, down)| {
            INPUT
                .split(|&c| c == b'\n')
                .filter(|line| !line.is_empty())
                .step_by(*down)
                .scan(0, |col, line| {
                    let is_tree = line[*col] == b'#';
                    *col = (*col + right) % line.len();
                    Some(is_tree as usize)
                })
                .sum::<usize>()
        })
        .product::<usize>();
    println!("{}", solution);
}
