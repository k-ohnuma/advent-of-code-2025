use std::cmp::max;

use itertools::Itertools;
use proconio::input;

fn parse_input(s: &str) -> (usize, usize) {
    let s = s.split(',').collect_vec();
    let a = s[0].parse::<usize>().unwrap();
    let b = s[1].parse::<usize>().unwrap();
    (a, b)
}

fn main() {
    let n = 496;
    input! {
        s: [String; n]
    }
    let ab = s.iter().map(|e| parse_input(e)).collect_vec();
    let mut ans = 0;
    for &(a1, b1) in ab.iter() {
        for &(a2, b2) in ab.iter() {
            ans = max(ans, (a1.abs_diff(a2) + 1) * (b1.abs_diff(b2) + 1));
        }
    }
    println!("{}", ans);
}
