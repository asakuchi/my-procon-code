// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let result = f(f(f(t) + t) + f(f(t)));

    println!("{}", result);
}

fn f(x: usize) -> usize {
    x * x + 2 * x + 3
}
