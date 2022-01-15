// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 010 - Score Sum Queries（★2）
///
fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut sum = vec![vec![0; n + 1]; 2];
    sum[0][0] = 0;
    sum[1][0] = 0;

    for i in 0..n {
        // println!("before sum:{:?}", sum);
        // println!("add:{}", cp[i].1);

        sum[0][i + 1] = sum[0][i];
        sum[1][i + 1] = sum[1][i];
        sum[cp[i].0 - 1][i + 1] += cp[i].1;
        // println!("after sum:{:?}", sum);
    }

    // println!("sum:{:?}", sum);

    for (l, r) in lr.iter() {
        let sum_0 = sum[0][*r] - sum[0][*l - 1];
        let sum_1 = sum[1][*r] - sum[1][*l - 1];

        println!("{} {}", sum_0, sum_1);
    }
}
