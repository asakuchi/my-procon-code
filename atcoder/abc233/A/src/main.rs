// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    if y > x {
        let result = y - x;
        let result = result as f64 / 10.0;

        println!("{}", result.ceil());
    } else {
        println!("{}", 0);
    }
}
