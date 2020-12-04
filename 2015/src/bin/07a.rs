use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/07.txt");

#[derive(Debug)]
enum Input {
    Value(u16),
    Wire(String),
}

impl Input {
    fn parse(name: &str) -> Self {
        match name.parse::<u16>() {
            Ok(value) => Self::Value(value),
            Err(_) => Self::Wire(name.to_owned()),
        }
    }
    fn value(&self, circuit: &Circuit) -> Option<u16> {
        match self {
            Self::Value(value) => Some(*value),
            Self::Wire(name) => circuit.value(&name),
        }
    }
}

#[derive(Debug)]
enum Gate {
    Repeat { input: Input },
    Not { input: Input },
    And { lhs: Input, rhs: Input },
    Or { lhs: Input, rhs: Input },
    Lshift { lhs: Input, rhs: Input },
    Rshift { lhs: Input, rhs: Input },
}

impl Gate {
    fn value(&self, circuit: &Circuit) -> Option<u16> {
        Some(match self {
            Self::Repeat { input } => input.value(&circuit)?,
            Self::Not { input } => input.value(&circuit)? ^ 0xFFFF,
            Self::And { lhs, rhs } => lhs.value(&circuit)? & rhs.value(&circuit)?,
            Self::Or { lhs, rhs } => lhs.value(&circuit)? | rhs.value(&circuit)?,
            Self::Lshift { lhs, rhs } => lhs.value(&circuit)? << rhs.value(&circuit)?,
            Self::Rshift { lhs, rhs } => lhs.value(&circuit)? >> rhs.value(&circuit)?,
        })
    }
}

use std::sync::Mutex;

#[derive(Default)]
struct Circuit {
    wires: HashMap<String, Gate>,
    cache: Mutex<HashMap<String, u16>>,
}

impl Circuit {
    fn wire(&mut self, name: &str, gate: Gate) {
        self.wires.insert(name.to_owned(), gate);
    }
    fn value(&self, name: &str) -> Option<u16> {
        let cache = self.cache.try_lock().unwrap();
        if let Some(value) = cache.get(name) {
            return Some(*value);
        }
        drop(cache);
        let gate = self.wires.get(name)?;
        let value = gate.value(self);
        self.cache
            .try_lock()
            .unwrap()
            .insert(name.to_owned(), value.unwrap());
        value
    }
}

const PATTERN: &str =
    r"^(?:(\d+|[a-z]+) )?(?:(NOT|AND|OR|LSHIFT|RSHIFT) )?(?:(\d+|[a-z]+)) -> ([a-z]+)$";

fn main() {
    let mut circuit = Circuit::default();
    let re = Regex::new(PATTERN).unwrap();
    for line in INPUT.trim().split("\n") {
        let m = re.captures(line).unwrap();
        match (
            m.get(1).map(|m| m.as_str()),
            m.get(2).map(|m| m.as_str()),
            &m[3],
            &m[4],
        ) {
            (None, None, input, out) => {
                let input = Input::parse(input);
                circuit.wire(out, Gate::Repeat { input });
            }
            (None, Some("NOT"), input, out) => {
                let input = Input::parse(input);
                circuit.wire(out, Gate::Not { input })
            }
            (Some(lhs), Some(op), rhs, out) => {
                let lhs = Input::parse(lhs);
                let rhs = Input::parse(rhs);
                circuit.wire(
                    out,
                    match op {
                        "AND" => Gate::And { lhs, rhs },
                        "OR" => Gate::Or { lhs, rhs },
                        "LSHIFT" => Gate::Lshift { lhs, rhs },
                        "RSHIFT" => Gate::Rshift { lhs, rhs },
                        op => unreachable!(op),
                    },
                );
            }
            _ => unreachable!(),
        }
    }
    println!("{}", circuit.value("a").unwrap());
}
