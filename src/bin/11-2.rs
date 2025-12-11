use std::{
    collections::HashMap,
    io::{self, Read},
};

use itertools::Itertools;
use regex::Regex;

fn parse_input(s: &str) -> (String, Vec<String>) {
    let reg = Regex::new(r"^([^:]+):(.+)$").unwrap();
    let cap = reg.captures(&s).unwrap();
    let l = cap[1].trim().to_string();
    let r = cap[2]
        .split(' ')
        .filter(|&e| !e.is_empty())
        .map(|e| e.to_string())
        .collect_vec();
    (l, r)
}

fn dfs(v: usize, set: usize, to: &Vec<Vec<usize>>, p: usize, dp: &mut Vec<Vec<Option<usize>>>, fi: usize, di: usize, oi: usize) -> usize {
    let mut new = set;
    if v == fi {
        new = new | 1 << 0;
    }
    if v == di {
        new = new | 1 << 1;
    }
    if let Some(v) = dp[v][new] {
        return v;
    }
    if v == oi {
        let ans = if new == 3 {1} else {0};
        dp[v][new] = Some(ans);
        return ans;
    }
    let mut ans = 0;
    for &v2 in to[v].iter() {
        if v2 == p {
            continue;
        }
        ans += dfs(v2, new, to, v, dp, fi, di, oi);
    }
    dp[v][new] = Some(ans);
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.lines().collect_vec();
    let s = s.iter().map(|e| parse_input(e)).collect_vec();
    let mut id = 0;
    let mut ids = HashMap::new();

    for p in s.iter() {
        let p1 = ids.get(&p.0);
        if p1.is_none() {
            ids.insert(p.0.to_string(), id);
            id += 1usize;
        }
        for word in p.1.iter() {
            let p1 = ids.get(word);
            if p1.is_none() {
                ids.insert(word.to_string(), id);
                id += 1usize;
            }
        }
    }
    let n = id;
    let mut to = vec![vec![]; n];
    for p in s.iter() {
        let &f = ids.get(&p.0).unwrap();
        for word in p.1.iter() {
            let &t = ids.get(word).unwrap();
            to[f].push(t);
        }
    }
    let &si = ids.get("svr").unwrap();
    let &di = ids.get("dac").unwrap();
    let &fi = ids.get("fft").unwrap();
    let &oi = ids.get("out").unwrap();

    let mut dp = vec![vec![None; 4]; n];
    let ans = dfs(si, 0, &to, usize::MAX, &mut dp, fi, di, oi);
    println!("{}", ans);
}

