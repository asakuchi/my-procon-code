// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: u128,
    }

    if n > 100 {
        println!("Yes");
        return;
    }

    let n_pow = n.pow(2);

    let two_pow = 2u128.pow(n as u32);

    // println!("{} {}", two_pow, n_pow);

    println!("{}", if two_pow > n_pow { "Yes" } else { "No" });
}
