// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        w: usize,
        h: usize,
        x: usize,
        y: usize,
    }

    let area = (w * h) as f64 / 2.;

    let multi = if 2 * x == w && 2 * y == h { 1 } else { 0 };

    println!("{} {}", area, multi);
}
