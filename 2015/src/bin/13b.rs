use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("../input/13.txt");

fn parse_rules(input: &str) -> Vec<(&str, &str, i64)> {
    let re =
        Regex::new(r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).$")
            .unwrap();
    input
        .lines()
        .filter_map(|c| re.captures(c))
        .map(|c| {
            (
                c.get(1).unwrap().as_str(),
                c.get(4).unwrap().as_str(),
                match &c[2] {
                    "gain" => 1,
                    "lose" => -1,
                    _ => unreachable!(),
                } * c[3].parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn main() {
    let rules = parse_rules(INPUT);
    let people = rules.iter().map(|(a, _, _)| a).unique().collect::<Vec<_>>();
    let map: HashMap<(&str, &str), i64> = rules.iter().map(|(a, b, x)| ((*a, *b), *x)).collect();
    let solution: i64 = people
        .iter()
        .permutations(people.len())
        .map(|perm| {
            perm.iter()
                .chain(&perm[0..1])
                .tuple_windows()
                .map(|(a, b)| map.get(&(a, b)).unwrap() + map.get(&(b, a)).unwrap())
                .collect::<Vec<i64>>()
        })
        .max_by_key(|perm: &Vec<i64>| perm.iter().sum::<i64>())
        .map(|perm| perm.iter().sum::<i64>() - perm.iter().min().unwrap())
        .unwrap();
    println!("{}", solution);
}
