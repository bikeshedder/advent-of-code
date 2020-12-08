const INPUT: &str = include_str!("../input/08.txt");

type Code<'a> = Vec<(&'a str, i32)>;

fn parse_code(source: &str) -> Code {
    source
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn run_code(code: &Code) -> i32 {
    let mut visited = vec![false; code.len()];
    let mut pc = 0usize;
    let mut acc = 0i32;
    while !visited[pc] {
        visited[pc] = true;
        match code[pc] {
            ("acc", x) => acc += x,
            ("jmp", x) => pc = ((pc as i32) + x - 1) as usize,
            _ => {}
        }
        pc += 1;
    }
    acc
}

fn main() {
    let code = parse_code(INPUT);
    let acc = run_code(&code);
    println!("{}", acc);
}
