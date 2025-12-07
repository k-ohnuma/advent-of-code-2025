use proconio::input;

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
    let row = 4;
    let col = 1000;
    input! {
        s: [[isize; col]; row],
        op: [String; col]
    }

    let s = transpose(s);
    let mut ans = 0;

    for (idx, row) in s.iter().enumerate() {
        let op = op[idx].to_owned();
        if op == "+" {
            let t = row.iter().fold(0, |acc, &now| acc + now);
            ans += t;
        } else {
            let t = row.iter().fold(1, |acc, &now| acc * now);
            ans += t;
        }
    }
    println!("{}", ans);
}
