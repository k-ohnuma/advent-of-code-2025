use std::{collections::{HashMap, VecDeque}, io::{self, Read}};

use itertools::Itertools;
use regex::Regex;

fn parse_input(s: &str) -> (Vec<char>, Vec<Vec<usize>>, Vec<usize>) {
    let reg1 = Regex::new(r"\[([.#]+)\]").unwrap();
    let reg2 = Regex::new(r"\(([^)]+)\)").unwrap();
    let reg3 = Regex::new(r"\{([^}]+)\}").unwrap();
    let mut v = vec![];
    for p in reg2.captures_iter(&s) {
        let t = &p[1];
        let nums = t.split(',').map(|e| e.parse::<usize>().unwrap()).collect_vec();
        v.push(nums);
    }
    let s1 = reg1.captures(&s).unwrap()[1].chars().collect_vec();
    let s3 = reg3.captures(&s).unwrap()[1].split(',').map(|e| e.parse::<usize>().unwrap()).collect_vec();
    
    (s1, v, s3)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input
        .lines().collect_vec();
    let t = s.iter().map(|e| parse_input(e)).collect_vec();

    let mut ans = 0;
    for (target, button, _) in t.iter() {
        let n = target.len();
        let mut dist = HashMap::new();
        dist.insert(vec!['.'; n], 0usize);
        let mut que = VecDeque::new();
        que.push_back(vec!['.'; n]);
        while !que.is_empty() {
            let v = que.pop_front().unwrap();
            let &now = dist.get(&v).unwrap();
            for row in button.iter() {
                let mut new = v.to_owned();
                for &idx in row.iter() {
                    new[idx] = if new[idx] == '#' {'.'} else {'#'};
                }
                let pre = dist.get(&new);
                if pre.is_some() {
                    continue;
                }
                dist.insert(new.to_owned(), now + 1);
                que.push_back(new);
            }
        }
        let &a = dist.get(target).unwrap();
        ans += a;
    }
    println!("{}", ans);
}
