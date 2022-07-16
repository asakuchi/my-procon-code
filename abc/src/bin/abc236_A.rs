// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
        a: usize,
        b: usize,
    }

    let pre = &s[..a - 1];
    let mid = &s[a..b - 1];
    let post = &s[b..];

    println!("{}{}{}{}{}", pre, &s[b - 1..b], mid, &s[a - 1..a], post);
}
