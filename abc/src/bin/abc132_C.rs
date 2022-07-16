// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut d: [usize; n],
    }

    d.sort();

    let result = d[d.len() / 2] - d[d.len() / 2 - 1];

    println!("{}", result);
}
