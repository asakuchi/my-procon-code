//!
//! 確率DP
//! https://compro.tsutaj.com/archive/180220_probability_dp.pdf
//!
//! 1から6までの整数が等確率に出るサイコロが1つある．
//! 出た目の数の和がK以上になるまでこのサイコロを振るとき、
//! サイコロを振る回数の期待値を求めよ．
//!

use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    // dp[i]
    let mut dp = vec![0.; k + 10];

    for i in (0..k).rev() {
        for j in 1..=6 {
            dp[i] += (dp[i + j] + 1.) / 6.
        }
    }

    println!("{}", dp[0]);
}
