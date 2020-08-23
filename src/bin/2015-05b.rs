const INPUT: &[u8] = include_bytes!("2015-05.txt");

fn main() {
    let output = INPUT
        .split(|&c| c == b'\n')
        .filter(|w| !w.is_empty())
        // two characters, twice in the word
        .filter(|w| {
            w.windows(2)
                .enumerate()
                .any(|(i, t)| w[i + 2..].windows(2).find(|&x| x == t).is_some())
        })
        // same letter separated by a letter
        .filter(|w| w.windows(3).any(|t| t[0] == t[2]))
        .count();
    println!("{:?}", output);
}
