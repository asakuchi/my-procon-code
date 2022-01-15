// -*- coding:utf-8-unix -*-

use proconio::input;
// use proconio::derive_readable;
// use proconio::marker::Chars;
// use itertools::izip;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        h: [usize; n],

    }

    let mut now = h[0];

    for i in 1..n {
        if h[i] > now {
            now = h[i];
        } else {
            break;
        }
    }

    println!("{}", now);
}
