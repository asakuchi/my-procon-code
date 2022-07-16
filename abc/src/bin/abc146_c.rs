// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: u128,
        b: u128,
        x: u128,
    }

    let mut high = 10u128.pow(9) + 1;
    let mut low = 0;
    let mut mid;

    loop {
        mid = (high + low) / 2;

        if mid == low {
            break;
        }

        if a * mid + b * mid.to_string().len() as u128 <= x {
            // 買える
            low = mid;
        } else {
            // 買えない
            high = mid;
        }
    }

    println!("{}", mid);
}
