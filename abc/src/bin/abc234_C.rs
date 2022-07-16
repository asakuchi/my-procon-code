// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        k: u128,
    }

    let answer = format!("{:b}", k);
    let answer = answer.replace("1", "2");

    println!("{}", answer);
}
