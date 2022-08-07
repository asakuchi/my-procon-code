//!
//! 確率DP
//! https://compro.tsutaj.com/archive/180220_probability_dp.pdf
//!
//! 1から6までの整数が等確率に出るサイコロが1つある.
//! このサイコロをN回振るとき、出た目の数の和がK以上になる確率を求めよ
//!

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    // dp[i][j]
    // i 回投げた時、出た目の数の和がjになる確率
    let mut dp = vec![vec![0.; n * 6 + 1]; n + 1];

    dp[0][0] = 1.;

    for i in 1..=n {
        for j in 0..=6 * n {
            for l in 1..=6 {
                if j >= l {
                    dp[i][j.min(k)] += dp[i - 1][j - l] / 6.;
                }
            }
        }
    }

    println!("{}", dp[n][k]);
}
