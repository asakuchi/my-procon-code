use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        c: [usize; n],
        r: [usize; n],
    }

    let mut candi = Vec::new();

    for i in 0..n {
        if c[i] == t {
            candi.push((r[i], i));
        }
    }

    if candi.len() == 0 {
        for i in 0..n {
            if c[i] == c[0] {
                candi.push((r[i], i));
            }
        }
    }

    candi.sort_by_key(|&(color, _i)| Reverse(color));

    println!("{}", candi[0].1 + 1);
}
