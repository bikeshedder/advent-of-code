const INPUT: &str = include_str!("2015-02.txt");

fn main() {
    let output: usize = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut v = line
                .split('x')
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            v.sort();
            let perimeter = 2 * v[0] + 2 * v[1];
            let volume = v[0] * v[1] * v[2];
            perimeter + volume
        })
        .sum();
    println!("{}", output);
}
