use itertools::Itertools;
use proconio::input;
use regex::Regex;

fn main() {
    let n = 4136;
    let mods = 100;
    let mut now = 50isize;
    input! {
        s: [String; n]
    }
    let reg = Regex::new("^([LR])(\\d{1,})$").unwrap();
    let s = s
        .iter()
        .map(|e| {
            let cap = reg.captures(e).unwrap();
            let c = cap[1].chars().next().unwrap();
            let num = cap[2].parse::<isize>().unwrap();
            (c, num)
        })
        .collect_vec();
    let mut ans = 0;
    for &(c, num) in s.iter() {
        match c {
            'L' => now = (now - num).rem_euclid(mods),
            'R' => now = (now + num).rem_euclid(mods),
            _ => unreachable!(""),
        }
        if now == 0 {
            ans += 1usize;
        }
    }
    println!("{}", ans);
}
#[cfg(test)]
mod test {
    use assert_cmd::Command;

    #[test]
    fn test() {
        let input = include_str!("../../input/day1/input.txt");
        let output = include_str!("../../input/day1/out-1.txt");
        #[allow(deprecated)]
        let mut cmd = Command::cargo_bin("1-1").unwrap();
        cmd.write_stdin(input).assert().success().stdout(output);
    }
}

