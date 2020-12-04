const INPUT: &str = include_str!("../input/08.txt");

fn decode(line: &str) -> Option<usize> {
    let mut it = line.chars();
    match it.next()? {
        '"' => {
            let mut count = 0;
            loop {
                let c = it.next()?;
                match c {
                    '"' => break,
                    '\\' => match it.next()? {
                        '\\' => count += 1,
                        '\"' => count += 1,
                        'x' => {
                            it.next()?;
                            it.next()?;
                            count += 1;
                        }
                        _ => return None,
                    },
                    _ => count += 1,
                }
            }
            if it.next().is_none() {
                Some(count)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn main() {
    let output = INPUT
        .trim()
        .split("\n")
        .map(|l| l.len() - decode(l).unwrap())
        .sum::<usize>();
    println!("{}", output);
}
