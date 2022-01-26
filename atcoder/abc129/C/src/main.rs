// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut p = vec![0 as u128; n + 1];

    p[0] = 1;

    if !a.contains(&1) {
        p[1] = 1;
    }

    for i in 2..=n {
        if a.contains(&i) {
            continue;
        }

        if p[i - 1] == 0 && p[i - 2] == 0 {
            break;
        }

        p[i] = (p[i - 1] + p[i - 2]) % 1_000_000_007;
    }

    println!("{}", p[n]);
}
