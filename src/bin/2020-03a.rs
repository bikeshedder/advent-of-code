const INPUT: &[u8] = include_bytes!("2020-03.txt");
const RIGHT: usize = 3;

fn main() {
    let solution = INPUT
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .scan(0, |col, line| {
            let is_tree = line[*col] == b'#';
            *col = (*col + RIGHT) % line.len();
            Some(is_tree as usize)
        })
        .sum::<usize>();
    println!("{}", solution);
}
