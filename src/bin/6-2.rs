use std::io::{self, Read};

use itertools::Itertools;

fn transpose<T: Clone + Copy + Default>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed = vec![vec![T::default(); rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j];
        }
    }

    transposed
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut lines: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect_vec()).collect();

    let m = lines.iter().map(|e| e.len()).max().unwrap();
    let mut op = lines.pop().unwrap();
    let mut lines = lines
        .into_iter()
        .map(|e| {
            e.iter()
                .map(|&e| e.to_string().parse::<usize>().unwrap_or(0))
                .collect_vec()
        })
        .collect_vec();
    for row in lines.iter_mut() {
        while row.len() < m {
            row.push(0);
        }
    }
    while op.len() < m {
        op.push(' ');
    }
    let s = transpose(lines);
    assert!(s.len() == m);
    let mut c_op = op[0];
    let mut pos = op.iter().positions(|&e| e != ' ').collect_vec();
    pos.reverse();
    assert!(c_op != ' ');
    let s = s
        .iter()
        .map(|e| {
            e.iter()
                .filter(|&&e| e != 0)
                .join("")
                .parse::<usize>()
                .unwrap_or(0)
        })
        .collect_vec();
    assert!(s.len() == m);
    let mut ans = 0;
    let mut v = vec![];
    for i in 0..m {
        let la = pos.iter().last();
        if let Some(&v) = la {
            if v == i {
                let ne = pos.pop().unwrap();
                c_op = op[ne];
            }
        }
        if s[i] == 0 {
            if c_op == '+' {
                let t = v.iter().sum::<usize>();
                ans += t;
            } else {
                let t = v.iter().fold(1, |acc, &now| acc * now);
                ans += t;
            }
            v.clear();
        } else {
            v.push(s[i]);
        }
    }
    if c_op == '+' {
        let t = v.iter().sum::<usize>();
        ans += t;
    } else {
        let t = v.iter().fold(1, |acc, &now| acc * now);
        ans += t;
    }
    println!("{}", ans);
}
