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

fn run_code(code: &Code, line_to_patch: usize) -> Option<i32> {
    let mut visited = vec![false; code.len()];
    let mut pc = 0usize;
    let mut acc = 0i32;
    while pc < code.len() && !visited[pc] {
        visited[pc] = true;
        match (code[pc], pc == line_to_patch) {
            (("acc", x), _) => acc += x,
            (("jmp", x), false) | (("nop", x), true) => pc = ((pc as i32) + x - 1) as usize,
            _ => {}
        }
        pc += 1;
    }
    if pc >= code.len() {
        Some(acc)
    } else {
        None
    }
}

fn main() {
    let code = parse_code(INPUT);
    let solution = (0..code.len())
        .filter_map(|line| run_code(&code, line))
        .next()
        .unwrap();
    println!("{}", solution);
}
