use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("../input/19.txt");

#[derive(Debug)]
enum Rule {
    Char(char),
    Or(Vec<Vec<usize>>),
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with("\"") {
            Rule::Char(s.chars().nth(1).unwrap())
        } else {
            Rule::Or(
                s.split(" | ")
                    .map(|x| x.split(" ").map(|n| n.parse().unwrap()).collect())
                    .collect(),
            )
        })
    }
}

#[derive(Debug)]
struct Rules(HashMap<usize, Rule>);

impl Rules {
    fn check_rule<'s>(&self, n: usize, s: &'s str) -> Option<&'s str> {
        match &self.0[&n] {
            Rule::Char(c) if s.chars().next().unwrap() == *c => Some(&s[1..]),
            Rule::Or(rules) => rules
                .iter()
                .find_map(|rules| rules.iter().try_fold(s, |s, n| self.check_rule(*n, s))),
            _ => None,
        }
    }
    fn matches<'s>(&self, s: &'s str) -> bool {
        matches!(self.check_rule(0, s), Some(s) if s.len() == 0)
    }
}

impl FromStr for Rules {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    let mut line_it = line.split(": ");
                    let n = line_it.next().unwrap().parse::<usize>().unwrap();
                    let rule = line_it.next().unwrap().parse::<Rule>().unwrap();
                    (n, rule)
                })
                .collect(),
        ))
    }
}

fn main() {
    let mut input_it = INPUT.split("\n\n");
    let rules: Rules = input_it.next().unwrap().parse().unwrap();
    let lines = input_it.next().unwrap().lines();
    let solution = lines.filter(|line| rules.matches(line)).count();
    println!("{}", solution);
}
