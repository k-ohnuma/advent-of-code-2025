use std::{cmp::Reverse, collections::BinaryHeap};

use ac_library::Dsu;
use itertools::Itertools;
use proconio::input;

fn parse_input(s: &str) -> (usize, usize, usize) {
    let s = s.split(',').collect_vec();
    let a = s[0].parse::<usize>().unwrap();
    let b = s[1].parse::<usize>().unwrap();
    let c = s[2].parse::<usize>().unwrap();
    (a, b, c)
}

fn main() {
    let count = 1000;
    let n = 1000;
    input! {
        s: [String; n]
    }
    let abc = s.iter().map(|e| parse_input(e)).collect_vec();
    let dist2 = |a: (usize, usize, usize), b: (usize, usize, usize)| {
        let (x1, y1, z1) = a;
        let (x2, y2, z2) = b;
        x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2) + z1.abs_diff(z2).pow(2)
    };
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        for j in i + 1..n {
            let d = dist2(abc[i], abc[j]);
            heap.push((Reverse(d), i, j));
        }
    }
    let mut uf = Dsu::new(n);
    for _ in 0..count {
        let (_, i, j) = heap.pop().unwrap();
        uf.merge(i, j);
    }
    let mut g = uf.groups().iter().map(|e| e.len()).collect_vec();
    g.sort_by_key(|&e| Reverse(e));
    assert!(g.len() >= 3);
    let ans = g[0] * g[1] * g[2];
    println!("{}", ans);
}
