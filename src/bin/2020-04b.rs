use regex::Regex;

const INPUT: &str = include_str!("2020-04.txt");
const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn check_year(s: &str, min: u64, max: u64) -> bool {
    s.parse::<u64>()
        .map(|x| x >= min && x <= max)
        .unwrap_or(false)
}

fn main() {
    let hgt_re = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();
    let solution = INPUT
        .trim()
        .split("\n\n")
        .map(|p| {
            p.split_whitespace()
                .filter_map(|f| {
                    let mut split = f.splitn(2, ":");
                    Some((split.next()?, split.next()?))
                })
                .map(|(k, v)| match k {
                    "byr" => check_year(v, 1920, 2002),
                    "iyr" => check_year(v, 2010, 2020),
                    "eyr" => check_year(v, 2020, 2030),
                    "hgt" => hgt_re
                        .captures(v)
                        .and_then(|c| {
                            let x = c[1].parse::<u64>().ok()?;
                            let u = &c[2];
                            match (x, u) {
                                (x, "cm") => Some(150 <= x && x <= 193),
                                (x, "in") => Some(59 <= x && x <= 76),
                                _ => None,
                            }
                        })
                        .unwrap_or(false),
                    "hcl" => hcl_re.find(v).is_some(),
                    "ecl" => EYE_COLORS.iter().find(|&&x| x == v).is_some(),
                    "pid" => pid_re.find(v).is_some(),
                    _ => false,
                })
                .filter(|&ok| ok)
                .count()
        })
        .filter(|&c| c >= 7)
        .count();
    println!("{:?}", solution);
}
