// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
// use proconio::derive_readable;
use proconio::marker::Chars;
// use itertools::izip;
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut list = vec![0];

    let mut prev = 0;

    for (i, &c) in s.iter().enumerate() {
        if c == 'L' {
            list.insert(prev, i + 1);

            prev = prev;
        } else {
            list.insert(prev + 1, i + 1);

            prev = prev + 1;
        }
    }

    let result: Vec<_> = list.iter().map(|x| x.to_string()).collect();

    // println!("{}", if yes { "Yes" } else { "No" });
    println!("{}", result.join(" "));
}
