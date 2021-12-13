const INPUT: &str = include_str!("../input/08.txt");

struct Entry {
    _signal_patterns: Vec<&'static str>,
    output_values: Vec<&'static str>,
}

impl Entry {
    fn parse(line: &'static str) -> Self {
        let mut it = line.split(" | ");
        Self {
            _signal_patterns: it.next().unwrap().split_whitespace().collect(),
            output_values: it.next().unwrap().split_whitespace().collect(),
        }
    }
}

fn main() {
    let lines = INPUT
        .trim()
        .split("\n")
        .map(Entry::parse)
        .collect::<Vec<_>>();
    let answer = lines.iter().map(|entry|
        entry.output_values.iter().filter(|p| {
            let l = p.len();
            l == 2 || l == 3 || l == 4 || l == 7
        }).count()
    ).sum::<usize>();
    println!("{}", answer);
}
