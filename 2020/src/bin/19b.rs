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
    fn check_rule<'s>(&self, n: usize, s: &'s str) -> Vec<&'s str> {
        match &self.0[&n] {
            Rule::Char(c) if s.chars().next() == Some(*c) => vec![&s[1..]],
            Rule::Or(rules) => rules
                .iter()
                .map(|rules| {
                    rules.iter().fold(vec![s], |s, n| {
                        s.iter().map(|s| self.check_rule(*n, s)).flatten().collect()
                    })
                })
                .flatten()
                .collect(),
            _ => vec![],
        }
    }
    fn matches<'s>(&self, s: &'s str) -> bool {
        self.check_rule(0, s)
            .iter()
            .find(|s| s.len() == 0)
            .is_some()
    }
    fn patch(&mut self, s: &str) {
        let (n, rule) = parse_rule_line(s);
        self.0.insert(n, rule);
    }
}

fn parse_rule_line(s: &str) -> (usize, Rule) {
    let mut line_it = s.split(": ");
    let n = line_it.next().unwrap().parse::<usize>().unwrap();
    let rule = line_it.next().unwrap().parse::<Rule>().unwrap();
    (n, rule)
}

impl FromStr for Rules {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(|line| parse_rule_line(line)).collect()))
    }
}

fn main() {
    let mut input_it = INPUT.split("\n\n");
    let mut rules: Rules = input_it.next().unwrap().parse().unwrap();
    rules.patch("8: 42 | 42 8");
    rules.patch("11: 42 31 | 42 11 31");
    let lines = input_it.next().unwrap().lines();
    let solution = lines.filter(|line| rules.matches(line)).count();
    println!("{}", solution);
}
