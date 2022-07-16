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
        n: i128,
    }

    if n >= (-2i128).pow(31) && n < (2i128).pow(31) {
        println!("Yes");
    } else {
        println!("No");
    }

    // println!("{}", if yes { "Yes" } else { "No" });
    // println!("{}", yes);
}
