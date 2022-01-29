// -*- coding:utf-8-unix -*-

use num::integer::lcm;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let b_not_c_divisor = b / c;
    let b_not_d_divisor = b / d;
    let b_not_cd_divisor = b / (lcm(c, d));

    let b_result = b - (b_not_c_divisor + b_not_d_divisor - b_not_cd_divisor);

    let a_not_c_divisor = (a - 1) / c;
    let a_not_d_divisor = (a - 1) / d;
    let a_not_cd_divisor = (a - 1) / (lcm(c, d));

    let a_result = (a - 1) - (a_not_c_divisor + a_not_d_divisor - a_not_cd_divisor);

    println!("{}", b_result - a_result);
}
