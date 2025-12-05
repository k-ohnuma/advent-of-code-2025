use std::cmp::max;

use itertools::Itertools;
use proconio::input;

fn parse_input(s: &str) -> Vec<(usize, usize)> {
    let hi = s.split(',').collect_vec();
    hi.into_iter()
        .map(|e| {
            let sp = e
                .split('-')
                .map(|e| e.parse::<usize>().unwrap())
                .collect_vec();
            (sp[0], sp[1])
        })
        .collect_vec()
}

fn main() {
    input! {
        s: String
    }
    let ab = parse_input(s.as_str());
    let ma = ab
        .iter()
        .map(|&e| max(e.0, e.1))
        .max()
        .unwrap()
        .to_string()
        .len();
    let mut ans = 0;
    for num in 1usize.. {
        let rep = format!("{}{}", num, num).parse::<usize>().unwrap();
        if rep.to_string().len() > ma {
            break;
        }
        for &(l, r) in ab.iter() {
            if l <= rep && rep <= r {
                ans += rep;
                break;
            }
        }
    }
    println!("{}", ans);
}
#[cfg(test)]
mod test {
    use assert_cmd::Command;

    #[test]
    fn test() {
        let input = include_str!("../../input/day2/input.txt");
        let output = include_str!("../../input/day2/out-1.txt");
        #[allow(deprecated)]
        let mut cmd = Command::cargo_bin("2-1").unwrap();
        cmd.write_stdin(input).assert().success().stdout(output);
    }
}

