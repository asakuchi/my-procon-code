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
        a: [usize; n],
    }

    let mut list = Vec::new();

    let mut degree = 0;

    list.push(0);

    for value in a {
        degree += value;

        if degree >= 360 {
            degree -= 360;
        }

        list.push(degree);
    }

    list.sort();

    let mut max = 360 - list[list.len() - 1];

    for i in 1..list.len() {
        let diff = list[i] - list[i - 1];
        max = std::cmp::max(max, diff);
    }

    println!("{}", max);
}
