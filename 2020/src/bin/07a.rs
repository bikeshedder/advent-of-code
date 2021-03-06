use std::collections::HashMap;

const INPUT: &str = include_str!("../input/07.txt");

type Rule<'a> = (&'a str, Vec<(usize, &'a str)>);

fn parse_rules(input: &str) -> impl Iterator<Item = Rule> {
    input
        .split(".\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut split = line.splitn(2, " bags contain ");
            let bag = split.next().unwrap();
            let content = match split.next().unwrap() {
                "no other bags" => Vec::new(),
                content => content
                    .split(", ")
                    .map(|part| {
                        let mut split = part.splitn(2, " ");
                        (
                            split.next().unwrap().parse::<usize>().unwrap(),
                            split.next().unwrap().rsplitn(2, " ").last().unwrap(),
                        )
                    })
                    .collect(),
            };
            (bag, content)
        })
}

type Rules<'a> = HashMap<&'a str, Vec<(usize, &'a str)>>;

fn can_contain(rules: &Rules, container: &str, bag: &str) -> bool {
    match rules.get(container) {
        Some(contents) => contents
            .iter()
            .find(|(_, b)| *b == bag || can_contain(rules, b, bag))
            .is_some(),
        None => false,
    }
}

fn main() {
    let rules = parse_rules(INPUT).collect::<Rules>();
    let solution = rules
        .keys()
        .filter(|bag| can_contain(&rules, bag, "shiny gold"))
        .count();
    println!("{}", solution);
}
