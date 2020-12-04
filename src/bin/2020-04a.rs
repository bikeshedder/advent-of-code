const INPUT: &str = include_str!("2020-04.txt");
const REQ_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let solution = INPUT
        .trim()
        .split("\n\n")
        .map(|p| {
            p.split_whitespace()
                .map(|f| f.split(":").next().unwrap())
                .filter(|k| REQ_FIELDS.iter().find(|&rf| rf == k).is_some())
                .count()
        })
        .filter(|&c| c >= 7)
        .count();
    println!("{:?}", solution);
}
