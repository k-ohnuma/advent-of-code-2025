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
        num: [usize; m]
    }

    let lr = range.iter().map(|e| parse_range(e)).collect_vec();
    let mut ans = 0;

    'l: for &num in num.iter() {
        for &(l, r) in lr.iter() {
            if l <= num && num <= r {
                ans += 1usize;
                continue 'l;
            }
        }
    }
    println!("{}", ans)
}
