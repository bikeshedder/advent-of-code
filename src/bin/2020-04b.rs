use regex::Regex;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("2020-04.txt");
const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn check_year(s: &str, min: u64, max: u64) -> bool {
    s.parse::<u64>()
        .map(|x| x >= min && x <= max)
        .unwrap_or(false)
}

fn check_height(s: &str) -> bool {
    scan_fmt!(s, "{d}{}", u64, String)
        .and_then(|(x, u)| match (x, u.as_str()) {
            (x, "cm") => Ok(150 <= x && x <= 193),
            (x, "in") => Ok(59 <= x && x <= 76),
            _ => Ok(false),
        })
        .unwrap_or(false)
}

fn main() {
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();
    let solution = INPUT
        .split("\n\n")
        .filter(|p| {
            p.split_whitespace()
                .filter_map(|f| {
                    let mut split = f.splitn(2, ":");
                    Some((split.next()?, split.next()?))
                })
                .map(|(k, v)| match k {
                    "byr" => check_year(v, 1920, 2002),
                    "iyr" => check_year(v, 2010, 2020),
                    "eyr" => check_year(v, 2020, 2030),
                    "hgt" => check_height(v),
                    "hcl" => hcl_re.find(v).is_some(),
                    "ecl" => EYE_COLORS.contains(&v),
                    "pid" => pid_re.find(v).is_some(),
                    _ => false,
                })
                .filter(|&ok| ok)
                .count()
                >= 7
        })
        .count();
    println!("{:?}", solution);
}
