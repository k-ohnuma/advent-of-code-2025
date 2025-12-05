use proconio::{input, marker::Chars};

fn main() {
    let h = 139;
    input! {
        s: [Chars; h]
    }
    let w = s[0].len();
    let didj = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '@' {
                continue;
            }
            let mut count = 0;
            for &(di, dj) in didj.iter() {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if s[ni][nj] == '@' {
                    count += 1usize;
                }
            }
            if count < 4 {
                ans += 1usize;
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
        let input = include_str!("../../input/day4/input.txt");
        let output = include_str!("../../input/day4/out-2.txt");
        #[allow(deprecated)]
        let mut cmd = Command::cargo_bin("4-2").unwrap();
        cmd.write_stdin(input).assert().success().stdout(output);
    }
}
