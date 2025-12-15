use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    let mut blocks = vec![];
    let count = 6;
    let case = 1000;
    let mut bs = vec![];
    for _ in 0..count {
        input! {
            _n: String,
            s: [Chars; 3]
        }
        bs.push(s.iter().flatten().filter(|&&e| e == '#').count());
        blocks.push(s);
    }

    let mut ans = 0usize;
    for _ in 0..case {
        input! {
            mut s: String,
            count: [usize; 6]
        }
        s.pop();
        let s = s.split('x').collect_vec();
        let h = s[0].parse::<usize>().unwrap();
        let w = s[1].parse::<usize>().unwrap();
        let cs = count.iter().sum::<usize>();

        let mut c = 0;
        for i in 0..6 {
            c += count[i] * bs[i];
        }

        if h / 3 * w / 3 >= cs {
            ans += 1usize;
        } else if c > h * w {
            continue;
        } else {
            // ここを通るケースがないのでこれでこの問題はとけます...
            panic!("ここを通ると他の解法が必要です...!!")
        }
    }
    println!("{}", ans);
}
