const INPUT: &str = include_str!("2015-04.txt");

fn main() {
    let secret = INPUT.trim();
    let output = (1..)
        .filter_map(|i| {
            let s = format!("{}{}", secret, i);
            let digest = md5::compute(s);
            if format!("{:?}", digest).starts_with("00000") {
                Some(i)
            } else {
                None
            }
        })
        .next()
        .unwrap();
    println!("{:?}", output);
}
