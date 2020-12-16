use std::{collections::HashSet, ops::Range};

use itertools::Itertools;

const INPUT: &str = include_str!("../input/16.txt");

#[derive(Debug)]
struct Field {
    name: String,
    ranges: Vec<Range<u32>>,
}

impl Field {
    fn check_value(&self, value: u32) -> bool {
        self.ranges
            .iter()
            .find(|range| range.contains(&value))
            .is_some()
    }
}

impl std::str::FromStr for Field {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut field_it = s.splitn(2, ": ");
        let field_name = field_it.next().unwrap();
        let ranges = field_it
            .next()
            .unwrap()
            .split(" or ")
            .map(|range| {
                let mut range_it = range.splitn(2, '-');
                let min = range_it.next().unwrap().parse::<u32>().unwrap();
                let max = range_it.next().unwrap().parse::<u32>().unwrap();
                min..max + 1
            })
            .collect::<Vec<_>>();
        Ok(Self {
            name: field_name.to_owned(),
            ranges: ranges,
        })
    }
}

struct Ticket {
    values: Vec<u32>,
}

impl std::str::FromStr for Ticket {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>(),
        })
    }
}

fn main() {
    let mut it = INPUT.split("\n\n");

    let fields = it
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse::<Field>().unwrap())
        .collect::<Vec<_>>();

    let own_ticket = it
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .parse::<Ticket>()
        .unwrap();

    let nearby_tickets = it
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| line.parse::<Ticket>().unwrap())
        .filter(|ticket| {
            ticket.values.iter().all(|value| {
                fields
                    .iter()
                    .find(|field| field.check_value(*value))
                    .is_some()
            })
        })
        .collect::<Vec<_>>();

    let field_mapping = fields
        .iter()
        .map(|field| {
            let indices = (0..fields.len())
                .filter(|i| {
                    nearby_tickets
                        .iter()
                        .all(|ticket| field.check_value(ticket.values[*i]))
                })
                .collect::<HashSet<_>>();
            (field, indices)
        })
        .sorted_by_key(|(_, indices)| indices.len())
        .scan(HashSet::new(), |used_indices, (field, indices)| {
            let index = *indices.difference(&used_indices).exactly_one().unwrap();
            used_indices.insert(index);
            Some((field, index))
        })
        .collect::<Vec<_>>();

    let solution = field_mapping
        .iter()
        .filter_map(|(field, index)| {
            if field.name.starts_with("departure") {
                Some(own_ticket.values[*index] as u64)
            } else {
                None
            }
        })
        .product::<u64>();

    println!("{}", solution);
}
