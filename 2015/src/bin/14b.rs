use regex::Regex;

const INPUT: &str = include_str!("../input/14.txt");
const SECONDS: u64 = 2503;

enum State {
    Flying(u64),
    Resting(u64),
}

struct Raindeer<'a> {
    #[allow(dead_code)]
    name: &'a str,
    speed: u64,
    duration: u64,
    rest: u64,
    state: State,
    distance: u64,
    score: u64,
}

fn main() {
    let re = Regex::new(
        r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$",
    )
    .unwrap();
    let mut raindeers = INPUT
        .lines()
        .filter_map(|line| {
            re.captures(line).map(|c| Raindeer {
                name: c.get(1).unwrap().as_str(),
                speed: c[2].parse::<u64>().unwrap(),
                duration: c[3].parse::<u64>().unwrap(),
                rest: c[4].parse::<u64>().unwrap(),
                state: State::Flying(0),
                distance: 0,
                score: 0,
            })
        })
        .collect::<Vec<_>>();
    for _ in 0..SECONDS {
        for raindeer in raindeers.iter_mut() {
            raindeer.state = match raindeer.state {
                State::Flying(t) => {
                    raindeer.distance += raindeer.speed;
                    if t + 1 < raindeer.duration {
                        State::Flying(t + 1)
                    } else {
                        State::Resting(0)
                    }
                }
                State::Resting(t) => {
                    if t + 1 < raindeer.rest {
                        State::Resting(t + 1)
                    } else {
                        State::Flying(0)
                    }
                }
            }
        }
        let max_distance = raindeers.iter().map(|r| r.distance).max().unwrap();
        for raindeer in raindeers.iter_mut() {
            if raindeer.distance == max_distance {
                raindeer.score += 1;
            }
        }
    }
    let solution = raindeers.iter().map(|r| r.score).max().unwrap();
    println!("{}", solution);
}
