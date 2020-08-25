use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("2015-09.txt");

fn main() {
    // step 1: find the number of unique destinations
    let dst = INPUT
        .trim()
        .split("\n")
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            vec![parts[0], parts[2]]
        })
        .flatten()
        .collect::<HashSet<_>>();
    // step 2: create mapping from name to index
    let dst_to_idx = dst
        .iter()
        .enumerate()
        .map(|t| (t.1, t.0))
        .collect::<HashMap<_, _>>();
    // step 3: create distance matrix
    let l = dst.len();
    let mut m = vec![vec![0; l]; l];
    for line in INPUT.trim().split("\n") {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let x = dst_to_idx[&parts[0]];
        let y = dst_to_idx[&parts[2]];
        let d = parts[4].parse::<usize>().unwrap();
        m[x][y] = d;
        m[y][x] = d;
    }
    // step 4: try all permutations
    let output: usize = (0..l)
        .permutations(l)
        .map(|p| p.iter().tuple_windows().map(|(&a, &b)| m[a][b]).sum())
        .max()
        .unwrap();
    println!("{}", output);
}
