const INPUT: &str = include_str!("../input/04.txt");
const REQ_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let solution = INPUT
        .split("\n\n")
        .filter(|p| {
            p.split_whitespace()
                .filter_map(|f| f.split(":").next())
                .filter(|k| REQ_FIELDS.contains(k))
                .count()
                >= 7
        })
        .count();
    println!("{:?}", solution);
}
