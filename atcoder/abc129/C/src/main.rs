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

    let mut broken = vec![false; n + 1];

    for value in a {
        broken[value] = true;
    }

    p[0] = 1;

    if !broken[1] {
        p[1] = 1;
    }

    for i in 2..=n {
        if broken[i] {
            continue;
        }

        p[i] = (p[i - 1] + p[i - 2]) % 1_000_000_007;
    }

    println!("{}", p[n]);
}
