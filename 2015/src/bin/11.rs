use itertools::Itertools;

const INPUT: &str = include_str!("../input/11.txt");
struct Password(usize);

impl Password {
    fn is_valid(&self) -> bool {
        let s = self.to_string();
        // straight of three letters
        if s.chars()
            .tuple_windows()
            .find(|(a, b, c)| (*a as usize + 1 == *b as usize) && (*b as usize + 1 == *c as usize))
            .is_none()
        {
            return false;
        }
        // must not contain i, o, l
        if s.chars()
            .find(|&c| c == 'i' || c == 'o' || c == 'l')
            .is_some()
        {
            return false;
        }
        // two non overlapping pairs like "aa", "bb" or "zz"
        if s.chars()
            .tuple_windows()
            .enumerate()
            .find(|(_, (a, b))| a == b)
            .and_then(|(i, _)| s[i + 2..].chars().tuple_windows().find(|(a, b)| a == b))
            .is_none()
        {
            return false;
        }
        true
    }
    fn increment(&mut self) {
        self.0 += 1;
    }
}

impl From<&str> for Password {
    fn from(s: &str) -> Self {
        Self(
            s.chars()
                .fold(0, |n, c| (26 * n as usize) + (c as usize - 'a' as usize)),
        )
    }
}

impl std::string::ToString for Password {
    fn to_string(&self) -> String {
        let mut x = self.0;
        let mut digits: Vec<u8> = Vec::new();
        loop {
            digits.push((x % 26) as u8);
            x -= x % 26;
            x /= 26;
            if x <= 0 {
                break;
            }
        }
        while digits.len() != 8 {
            digits.push(0);
        }
        digits.reverse();
        digits.iter().map(|d| ('a' as u8 + *d) as char).collect()
    }
}

fn main() {
    let mut pw = Password::from(INPUT.trim());
    for _ in 0..2 {
        loop {
            pw.increment();
            if pw.is_valid() {
                break;
            }
        }
        println!("{}", pw.to_string());
    }
}
