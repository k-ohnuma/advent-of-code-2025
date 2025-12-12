use std::{
    cmp::min,
    io::{self, Read},
};

use itertools::Itertools;
use num::integer::gcd;
use regex::Regex;

fn parse_input(s: &str) -> (Vec<char>, Vec<Vec<usize>>, Vec<isize>) {
    let reg1 = Regex::new(r"\[([.#]+)\]").unwrap();
    let reg2 = Regex::new(r"\(([^)]+)\)").unwrap();
    let reg3 = Regex::new(r"\{([^}]+)\}").unwrap();
    let mut v = vec![];
    for p in reg2.captures_iter(&s) {
        let t = &p[1];
        let nums = t
            .split(',')
            .map(|e| e.parse::<usize>().unwrap())
            .collect_vec();
        v.push(nums);
    }
    let s1 = reg1.captures(&s).unwrap()[1].chars().collect_vec();
    let s3 = reg3.captures(&s).unwrap()[1]
        .split(',')
        .map(|e| e.parse::<isize>().unwrap())
        .collect_vec();

    (s1, v, s3)
}

fn gauss(v: &mut Vec<Vec<isize>>) -> (usize, Vec<usize>) {
    let n = v.len();
    let m = v[0].len() - 1;
    let mut rank = 0;
    let mut cs = vec![];
    for col in 0..m {
        let mut t = usize::MAX;
        for row in rank..n {
            if v[row][col] != 0 {
                t = row;
                break;
            }
        }
        if t == usize::MAX {
            continue;
        }
        v.swap(rank, t);
        cs.push(col);
        let p = v[rank][col];
        for row in (rank + 1)..n {
            if v[row][col] == 0 {
                continue;
            }
            let g = gcd(p, v[row][col]);
            let t = v[row][col] / g;
            let tt = p / g;
            for j in col..=m {
                v[row][j] = v[row][j] * tt - v[rank][j] * t;
            }
        }
        rank += 1usize;
    }
    (rank, cs)
}

fn dfs(
    idx: usize,
    now: &mut Vec<isize>,
    nowsum: isize,
    table: &Vec<Vec<isize>>,
    v: &Vec<usize>,
    rv: &Vec<usize>,
    ans: &mut isize,
    lim: isize,
) {
    if nowsum >= *ans {
        return;
    }
    let m = table[0].len() - 1;
    let mm = rv.len();
    if idx == v.len() {
        for rank in (0..mm).rev() {
            let col = rv[rank];
            let mut right = table[rank][m];
            for j in (col + 1)..m {
                right -= table[rank][j] * now[j];
            }
            let tm = table[rank][col];

            if tm == 0 {
                if right != 0 {
                    return;
                } else {
                    continue;
                }
            }

            if right % tm != 0 {
                return;
            }
            let val = right / tm;
            if val < 0 {
                return;
            }
            now[col] = val;
        }
        let t = now.iter().sum::<isize>();
        *ans = min(*ans, t);
        return;
    }
    for c in 0..=lim {
        now[v[idx]] = c;
        dfs(idx + 1, now, nowsum + c, table, v, rv, ans, lim);
    }
}

fn solve(button: Vec<Vec<usize>>, target: Vec<isize>) -> isize {
    let m = button.len();
    let n = button.iter().flatten().max().unwrap() + 1;

    let mut table = vec![vec![0; m + 1]; n];
    for i in 0..m {
        for &j in button[i].iter() {
            table[j][i] = 1isize;
        }
    }
    for i in 0..n {
        table[i][m] = target[i];
    }

    let (_, cs) = gauss(&mut table);
    let t = (0..m).filter(|&e| !cs.contains(&e)).collect_vec();
    let rt = (0..m).filter(|&e| cs.contains(&e)).collect_vec();
    let lim = target.iter().sum::<isize>();
    let mut ans = isize::MAX;
    let mut now = vec![0; m];
    dfs(0, &mut now, 0, &table, &t, &rt, &mut ans, lim);
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.lines().collect_vec();
    let t = s.iter().map(|e| parse_input(e)).collect_vec();

    let mut ans = 0;
    for (_, button, target) in t {
        ans += solve(button, target);
    }
    println!("{}", ans);
}
