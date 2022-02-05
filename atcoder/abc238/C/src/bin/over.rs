// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
// use proconio::derive_readable;
// use proconio::marker::Chars;
// use itertools::izip;
// use itertools::Itertools;

// 時間オーバー

#[fastout]
fn main() {
    input! {
        n: u128,
    }

    // println!("{}", keta);

    let mut answer = 0;

    for i in 1..=n {
        let keta = i.to_string().len();
        let f = i % 998244353 - (10u128.pow((keta - 1) as u32) % 998244353) + 1;

        // println!("{} {}", i, f);

        answer += f;
    }
    println!("{}", answer);
}
