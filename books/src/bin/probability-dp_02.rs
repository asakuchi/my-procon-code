//!
//! 確率DP
//! https://compro.tsutaj.com/archive/180220_probability_dp.pdf
//!
//! コイントスをN回行うとき、表または裏がK回以上連続で出る確率を求めよ
//!

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    // dp[i][j]
    // i 回投げた時、表がj回連続になる確率
    let mut dp = vec![vec![0.; n + 1]; n + 1];

    dp[1][1] = 1.;

    for i in 2..=n {
        // 1回連続
        for j in 1..k {
            dp[i][1] += dp[i - 1][j] / 2.;
        }

        // 2回~k回連続
        for j in 1..k {
            dp[i][j] += dp[i - 1][j - 1] / 2.;
        }
    }

    let mut sum = 0.;

    for i in 0..k {
        sum += dp[n][i];
    }

    println!("{}", 1. - sum);
}
