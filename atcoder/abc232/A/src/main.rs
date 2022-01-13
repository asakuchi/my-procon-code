// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let a: &i32 = &s[0..1].parse().unwrap();
    let b: &i32 = &s[2..3].parse().unwrap();

    println!("{}", a * b);
}
