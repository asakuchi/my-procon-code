// -*- coding:utf-8-unix -*-

use proconio::input;
// use proconio::derive_readable;
// use proconio::marker::Chars;
// use itertools::izip;
// use itertools::Itertools;

fn main() {
    input! {

        s: String,

    }
    // let x: usize = s[0..1].parse().unwrap();

    let a = &s[0..1];
    let b = &s[1..2];
    let c = &s[2..3];

    let abc: usize = format!("{}{}{}", a, b, c).parse().unwrap();
    let bca: usize = format!("{}{}{}", b, c, a).parse().unwrap();
    let cab: usize = format!("{}{}{}", c, a, b).parse().unwrap();

    // println!("{}", if yes { "Yes" } else { "No" });

    println!("{}", abc + bca + cab);
}
