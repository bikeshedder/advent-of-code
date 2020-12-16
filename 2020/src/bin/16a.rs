use std::ops::Range;

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
        Ok(Field {
            name: field_name.to_owned(),
            ranges: ranges,
        })
    }
}

fn main() {
    let mut it = INPUT.split("\n\n");

    // parse fields
    let fields = it
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse::<Field>().unwrap())
        .collect::<Vec<_>>();

    it.next(); // skip our own ticket

    let nearby_tickets = it
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let solution = nearby_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .map(|value| {
                    if fields
                        .iter()
                        .find(|field| field.check_value(*value))
                        .is_none()
                    {
                        *value
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{}", solution);
}
