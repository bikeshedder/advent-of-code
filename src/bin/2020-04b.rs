use itertools::Itertools;
use regex::Regex;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("2020-04.txt");

fn check_year(s: &str, min: u64, max: u64) -> bool {
    matches!(s.parse::<u64>(), Ok(x) if x >= min && x <= max)
}

fn check_height(s: &str) -> bool {
    scan_fmt!(s, "{d}{}", u64, String)
        .map(|(x, u)| match (x, u.as_str()) {
            (x, "cm") => 150 <= x && x <= 193,
            (x, "in") => 59 <= x && x <= 76,
            _ => false,
        })
        .unwrap_or(false)
}

fn main() {
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let ecl_re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();
    let solution = INPUT
        .split("\n\n")
        .filter(|p| {
            p.split_whitespace()
                .filter_map(|f| f.splitn(2, ":").collect_tuple::<(_, _)>())
                .filter(|(k, v)| match *k {
                    "byr" => check_year(v, 1920, 2002),
                    "iyr" => check_year(v, 2010, 2020),
                    "eyr" => check_year(v, 2020, 2030),
                    "hgt" => check_height(v),
                    "hcl" => hcl_re.is_match(v),
                    "ecl" => ecl_re.is_match(v),
                    "pid" => pid_re.is_match(v),
                    _ => false,
                })
                .count()
                >= 7
        })
        .count();
    println!("{:?}", solution);
}
