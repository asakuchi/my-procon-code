// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
// use proconio::derive_readable;
// use proconio::marker::Chars;
// use itertools::izip;
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_by(|a, b| a.1.cmp(&b.1));

    // println!("{:?}", ab);

    let mut result = true;
    let mut total = 0;

    for (a, b) in &ab {
        total += a;

        if total > *b {
            result = false;
            break;
        }
    }

    println!("{}", if result { "Yes" } else { "No" });
}
