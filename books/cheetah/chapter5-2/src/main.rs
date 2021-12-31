// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize, // numbers の要素数
        numbers: [usize;n],
    }

    let mut max = 0;

    for i in 0..n {
        let mut product = 1;

        for j in 0..n {
            if i != j {
                product *= numbers[j];
            } else {
                product *= numbers[j] + 1;
            }
        }

        max = std::cmp::max(max, product);
    }

    println!("{}", max);
}
