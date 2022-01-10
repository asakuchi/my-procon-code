// -*- coding:utf-8-unix -*-

use itertools::izip;
use proconio::input;

///
/// 第8章 円の国家群
///
fn main() {
    input! {
        n: u32, // 要素数
        x: [i32; n],
        y: [i32; n],
        r: [u32; n],
        xy1: (i32, i32),
        xy2: (i32, i32),
    }

    let mut count_1 = Vec::new();
    let mut count_2 = Vec::new();

    for (index, &x_value, &y_value, &r_value) in izip!((0..n), x.iter(), y.iter(), r.iter()) {
        let distance_pow2 = (x_value - xy1.0).pow(2) + (y_value - xy1.1).pow(2);

        if distance_pow2 < r_value.pow(2) as i32 {
            count_1.push(index);
        }

        let distance_pow2 = (x_value - xy2.0).pow(2) + (y_value - xy2.1).pow(2);

        if distance_pow2 < r_value.pow(2) as i32 {
            count_2.push(index);
        }
    }

    let mut count = 0;

    for num in count_1.iter() {
        if !count_2.contains(&num) {
            count += 1;
        }
    }

    for num in count_2.iter() {
        if !count_1.contains(&num) {
            count += 1;
        }
    }

    println!("{}", count);
}
