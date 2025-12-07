use std::mem::swap;

use proconio::{input, marker::Chars};

fn main() {
    let n = 142;
    input! {
        s: [Chars; n]
    }
    let m = s[0].len();
    let mut start = (usize::MAX, usize::MAX);
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == 'S' {
                start = (i, j);
            }
        }
    }
    let mut now = vec![false; m];
    now[start.1] = true;
    let mut ans = 0usize;
    for i in start.0..n {
        let mut new = vec![false; m];
        for j in 0..m {
            if !now[j] {
                continue;
            }
            let djs = if s[i][j] == '^' { vec![-1, 1] } else { vec![0] };
            if s[i][j] == '^' {
                ans += 1usize;
            }
            for &dj in djs.iter() {
                let nj = j as isize + dj;
                if nj < 0 || nj >= m as isize {
                    continue;
                }
                let nj = nj as usize;
                new[nj] = true;
            }
        }
        swap(&mut now, &mut new);
    }

    println!("{}", ans);
}
