const INPUT: &str = include_str!("../input/18.txt");

#[derive(Clone, Debug)]
enum Op {
    Add,
    Multiply,
}

impl Op {
    fn exec(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Multiply => lhs * rhs,
        }
    }
}

#[derive(Clone, Debug)]
enum Token {
    Num(u64),
    Op(Op),
    ParenLeft,
    ParenRight,
}

struct MathParser<I: Iterator<Item = char>>(I);

impl<I: Iterator<Item = char>> MathParser<I> {
    fn new(it: I) -> Self {
        Self(it)
    }
}

impl<I: Iterator<Item = char>> Iterator for MathParser<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.0.next() {
                Some('+') => Some(Token::Op(Op::Add)),
                Some('*') => Some(Token::Op(Op::Multiply)),
                Some('(') => Some(Token::ParenLeft),
                Some(')') => Some(Token::ParenRight),
                Some(c) if c.is_whitespace() => continue,
                Some(c) => Some(Token::Num(c.to_digit(10).unwrap() as u64)),
                None => None,
            };
        }
    }
}

fn eval(s: &str) -> u64 {
    let parser = MathParser::new(s.chars());
    let mut stack = Vec::<Token>::new();
    for token in parser {
        let top = stack.last().cloned();
        match (top, token) {
            (Some(Token::Op(op)), Token::Num(n)) => {
                stack.pop();
                match stack.pop() {
                    Some(Token::Num(x)) => stack.push(Token::Num(op.exec(x, n))),
                    _ => unreachable!(),
                }
            }
            (Some(Token::Num(n)), Token::ParenRight) => {
                stack.pop();
                match stack.pop() {
                    Some(Token::ParenLeft) => match stack.pop() {
                        Some(Token::Op(op)) => match stack.pop() {
                            Some(Token::Num(x)) => stack.push(Token::Num(op.exec(x, n))),
                            z => unreachable!("{:?}", z),
                        },
                        Some(Token::ParenLeft) => {
                            stack.push(Token::ParenLeft);
                            stack.push(Token::Num(n));
                        }
                        None => stack.push(Token::Num(n)),
                        z => unreachable!("{:?}", z),
                    },
                    None => stack.push(Token::Num(n)),
                    z => unreachable!("{:?}", z),
                }
            }
            (_, Token::Num(n)) => stack.push(Token::Num(n)),
            (_, Token::Op(op)) => stack.push(Token::Op(op)),
            (_, Token::ParenLeft) => stack.push(Token::ParenLeft),
            z => unreachable!("{:?}", z),
        }
    }
    assert_eq!(stack.len(), 1);
    if let Some(Token::Num(solution)) = stack.pop() {
        solution
    } else {
        unreachable!()
    }
}

fn main() {
    let solution = INPUT.lines().map(|line| eval(line)).sum::<u64>();
    println!("{}", solution);
}
