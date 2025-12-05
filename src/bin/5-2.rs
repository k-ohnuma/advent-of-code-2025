use std::cmp::max;

use itertools::Itertools;
use proconio::input;

fn parse_range(s: &str) -> (usize, usize) {
    let hi = s.split('-').collect_vec();
    let a = hi[0].parse::<usize>().unwrap();
    let b = hi[1].parse::<usize>().unwrap();
    (a, b)
}

fn main() {
    let n = 177;
    let m = 1000;
    input! {
        range: [String; n],
        _num: [usize; m]
    }

    let mut lr = range.iter().map(|e| parse_range(e)).collect_vec();
    lr.sort_by_key(|&e| (e.0, e.1));
    let mut now = (lr[0].0, lr[0].0);
    let mut ans = 0;
    for &(l, r) in lr.iter() {
        let (pl, pr) = now;
        assert!(pl <= l);
        if pr < l {
            ans += pr - pl + 1;
            now = (l, r);
            continue;
        }
        now.1 = max(r, pr);
    }
    ans += now.1 - now.0 + 1;
    println!("{}", ans);
}
