use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
};

use itertools::Itertools;
use proconio::input;

fn parse_input(s: &str) -> (usize, usize) {
    let s = s.split(',').collect_vec();
    let a = s[0].parse::<usize>().unwrap();
    let b = s[1].parse::<usize>().unwrap();
    (b, a)
}

fn main() {
    let n = 496;
    input! {
        s: [String; n]
    }
    let ab = s.iter().map(|e| parse_input(e)).collect_vec();
    let mut is = ab.iter().map(|e| e.0).collect_vec();
    let mut js = ab.iter().map(|e| e.1).collect_vec();
    is.sort();
    is.dedup();
    js.sort();
    js.dedup();
    let imap = {
        let mut imap = HashMap::new();
        let mut id = 1;
        for &num in is.iter() {
            imap.insert(num, id);
            id += 1usize;
        }
        imap
    };
    let jmap = {
        let mut jmap = HashMap::new();
        let mut id = 1;
        for &num in js.iter() {
            jmap.insert(num, id);
            id += 1usize;
        }
        jmap
    };
    let h = imap.len() + 2;
    let w = jmap.len() + 2;

    let mut table = vec![vec![false; w]; h];
    let mut idx = vec![];
    for &(a, b) in ab.iter() {
        let &aa = imap.get(&a).unwrap();
        let &bb = jmap.get(&b).unwrap();
        idx.push((aa, bb));
    }
    for i in 0..n {
        let (a1, b1) = idx[i];
        let (a2, b2) = idx[(i + 1) % n];
        if a1 == a2 {
            let l = min(b1, b2);
            let r = max(b1, b2);
            for col in l..=r {
                table[a1][col] = true;
            }
        } else {
            let l = min(a1, a2);
            let r = max(a1, a2);
            for row in l..=r {
                table[row][b1] = true;
            }
        }
    }
    let mut que = VecDeque::new();
    let mut blank = vec![vec![0; w]; h];
    que.push_back((0, 0));

    let didj = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
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
            if table[ni][nj] {
                continue;
            }
            if blank[ni][nj] == 1 {
                continue;
            }
            blank[ni][nj] = 1usize;
            que.push_back((ni, nj));
        }
    }
    let mut acc = vec![vec![0usize; w]; h];
    for i in 0..h {
        for j in 0..w {
            if blank[i][j] == 0 {
                continue;
            }
            acc[i][j] = 1;
        }
    }
    for i in 0..h  {
        for j in 0..w - 1 {
            acc[i][j + 1] += acc[i][j];
        }
    }
    for j in 0..w   {
        for i in 0..h - 1 {
            acc[i + 1][j] += acc[i][j];
        }
    }

    let mut ans = 0;
    for &(i1, j1) in idx.iter() {
        for &(i2, j2) in idx.iter() {
            let li = min(i1, i2);
            let ri = max(i1, i2);
            let lj = min(j1, j2);
            let rj = max(j1, j2);
            let bc = acc[ri][rj] + acc[li - 1][lj - 1] - acc[ri][lj - 1] - acc[li - 1][rj];
            if bc != 0 {
                continue;
            }
            let idiff = is[max(li, ri) - 1] - is[min(i1, i2) - 1] + 1;
            let jdiff = js[max(j1, j2) - 1] - js[min(j1, j2) - 1] + 1;
            ans = max(ans, idiff * jdiff);
        }
    }
    println!("{}", ans);
}
