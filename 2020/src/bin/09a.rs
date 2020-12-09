use std::collections::VecDeque;

const INPUT: &str = include_str!("../input/09.txt");
const SIZE: usize = 25;

fn main() {
    let mut it = INPUT.lines().map(|line| str::parse(line).unwrap());
    let buf: VecDeque<u64> = it.by_ref().take(SIZE).collect();
    let solution = it
        .scan(buf, |buf, x| {
            let ok = buf
                .iter()
                .enumerate()
                .find(|(i, a)| buf.iter().skip(i + 1).find(|b| x == *a + *b).is_some())
                .is_some();
            buf.pop_front();
            buf.push_back(x);
            Some((ok, x))
        })
        .filter(|(ok, _)| !ok)
        .next()
        .unwrap()
        .1;
    println!("{}", solution);
}
