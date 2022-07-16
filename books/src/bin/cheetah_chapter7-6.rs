// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 第7章 ビジネス握手
///
fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![-1 as i64; n / 2 + 1];

    dp[0] = 1;

    for i in 1..=n / 2 {
        dp[i] = 0;

        for j in 0..i {
            dp[i] += dp[j] * dp[i - j - 1];
        }
    }

    println!("{}", dp[n / 2]);
}
