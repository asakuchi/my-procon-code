// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
    }

    if r - l <= 2019 {
        let mut min = 2020;

        for i in l..=r {
            for j in i + 1..=r {
                let num = (i * j) % 2019;

                min = std::cmp::min(min, num);
            }
        }

        println!("{}", min);
    } else {
        let mut min = 2020;

        for i in 0..2019 {
            for j in i + 1..=2019 {
                let num = (i * j) % 2019;

                min = std::cmp::min(min, num);
            }
        }

        println!("{}", min);
    }
}
