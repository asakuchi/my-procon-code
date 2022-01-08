// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
        s: String,
    }

    let l = l - 1;

    let first = &s[..l];
    let second: Vec<String> = s[l..r].chars().rev().map(|x| x.to_string()).collect();
    let second = second.join("");
    let third = &s[r..];

    println!("{}{}{}", first, second, third);
}
