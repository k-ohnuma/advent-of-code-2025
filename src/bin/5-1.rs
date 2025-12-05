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

#[cfg(test)]
mod test {
    use assert_cmd::Command;

    #[test]
    fn test() {
        let input = include_str!("../../input/day5/input.txt");
        let output = include_str!("../../input/day5/out-1.txt");
        #[allow(deprecated)]
        let mut cmd = Command::cargo_bin("5-1").unwrap();
        cmd.write_stdin(input).assert().success().stdout(output);
    }
}
