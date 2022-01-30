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
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    for i in 0..w {
        let mut first = true;
        for j in 0..h {
            if !first {
                print!(" ");
            }
            first = false;

            print!("{}", a[j][i]);
        }
        println!();
    }
}
