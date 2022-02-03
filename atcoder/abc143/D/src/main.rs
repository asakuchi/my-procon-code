// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
use superslice::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut l: [usize; n],
    }

    l.sort();

    let mut count = 0;

    for i in 0..n {
        for j in i + 1..n {
            let k = l.lower_bound(&(l[i] + l[j]));
            count += k - j - 1;
        }
    }

    println!("{}", count);
}
