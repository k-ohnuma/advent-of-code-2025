use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    let h = 139;
    input! {
        s: [Chars; h]
    }
    let w = s[0].len();
    let didj = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut ans = vec![vec![false; w]; h];
    let mut c = vec![vec![0; w]; h];
    let mut que = VecDeque::new();
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
            c[i][j] = count;
            if count < 4 {
                que.push_back((i, j));
                ans[i][j] = true;
            }
        }
    }
    while !que.is_empty() {
        let (i, j) = que.pop_front().unwrap();
        for &(di, dj) in didj.iter() {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if ans[ni][nj] || c[ni][nj] == 0 || s[ni][nj] == '.' {
                continue;
            }
            c[ni][nj] -= 1;
            if c[ni][nj] < 4 {
                ans[ni][nj] = true;
                que.push_back((ni, nj));
            }
        }
    }
    let ans = ans.iter().flatten().filter(|&&e|e).count();
    println!("{}", ans);
}
