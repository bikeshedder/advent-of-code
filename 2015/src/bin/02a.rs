const INPUT: &str = include_str!("../input/02.txt");

fn main() {
    let output: usize = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let v = line
                .split('x')
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let l = v.get(0).unwrap();
            let w = v.get(1).unwrap();
            let h = v.get(2).unwrap();
            let a1 = l * w;
            let a2 = w * h;
            let a3 = h * l;
            let am = std::cmp::min(std::cmp::min(a1, a2), a3);
            2 * a1 + 2 * a2 + 2 * a3 + am
        })
        .sum();
    println!("{}", output);
}
