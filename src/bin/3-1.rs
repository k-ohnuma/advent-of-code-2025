use std::{cmp::max, collections::BTreeMap};

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    let n = 200;
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
        let mut map = BTreeMap::new();
        for &num in row.iter() {
            map.entry(num).and_modify(|e| *e += 1).or_insert(1usize);
        }
        let mut t = 0;
        for &num in row.iter() {
            let &pre = map.get(&num).unwrap();
            if pre == 1 {
                map.remove(&num);
            } else {
                map.insert(num, pre - 1);
            }
            let la = map.last_key_value();
            if la.is_none() {
                continue;
            }
            let &la = la.unwrap().0;
            let now = num * 10 + la;
            t = max(t, now);
        }
        ans += t;
    }
    println!("{}", ans);
}
#[cfg(test)]
mod test {
    use assert_cmd::Command;

    #[test]
    fn test() {
        let input = include_str!("../../input/day3/input.txt");
        let output = include_str!("../../input/day3/out-1.txt");
        #[allow(deprecated)]
        let mut cmd = Command::cargo_bin("3-1").unwrap();
        cmd.write_stdin(input).assert().success().stdout(output);
    }
}
