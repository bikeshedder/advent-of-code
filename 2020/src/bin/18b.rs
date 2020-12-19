const INPUT: &str = include_str!("../input/18.txt");
#[derive(Clone, Debug)]
enum Op {
    Add,
    Multiply,
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
        stack.push(token);
        while stack.len() >= 3 {
            let top = &stack[stack.len() - 3..stack.len()];
            match (top[0].clone(), &top[1].clone(), &top[2].clone()) {
                (Token::Num(a), Token::Op(Op::Add), Token::Num(b)) => {
                    stack.truncate(stack.len() - 3);
                    stack.push(Token::Num(a + b));
                }
                (_, _, Token::ParenRight) => {
                    let mut p = 1;
                    while let Some(token) = stack.pop() {
                        match token {
                            Token::Num(n) => p *= n,
                            Token::ParenLeft => break,
                            Token::Op(Op::Multiply) => {}
                            Token::ParenRight => {}
                            token => unreachable!("unexpected token {:?}", token),
                        }
                    }
                    stack.push(Token::Num(p));
                }
                _ => break,
            }
        }
    }
    stack
        .iter()
        .map(|token| if let Token::Num(n) = token { *n } else { 1 })
        .product::<u64>()
}

fn main() {
    let solution = INPUT.lines().map(|line| eval(line)).sum::<u64>();
    println!("{}", solution);
}
