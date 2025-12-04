use std::{cmp::Reverse, collections::BTreeSet};

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    let n = 200;
    let choose = 12;
    input! {
        s: [Chars; n]
    }
    let nums = s
        .iter()
        .map(|e| {
            e.iter()
                .map(|e| e.to_string().parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut ans = 0;
    for row in nums.iter() {
        let mut set = BTreeSet::new();
        let fe = row.len() - choose;
        for i in 0..=fe {
            set.insert((Reverse(row[i]), i));
        }
        let mut now = 0;
        let mut a: Vec<usize> = vec![];
        while a.len() < choose {
            let fi = set.pop_first().unwrap();
            if fi.1 < now {
                continue;
            }
            now = fi.1;
            let fi = fi.0.0;
            a.push(fi);
            if fe + a.len() < row.len() {
                set.insert((Reverse(row[fe + a.len()]), fe + a.len()));
            }
        }
        assert_eq!(a.len(), choose);
        let num = a.iter().join("").parse::<usize>().unwrap();
        ans += num;
    }
    println!("{}", ans);
}

