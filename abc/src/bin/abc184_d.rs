//!
//! 確率DP/期待値DP
//!

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    // 金貨i枚、銀貨j枚、銅貨k枚が袋に入っているとき、硬貨が100枚になるまでの操作回数の期待値
    let mut dp = vec![vec![vec![Option::None; 101]; 101]; 101];

    let result = rec(a, b, c, &mut dp);

    println!("{}", result);
}

fn rec(a: usize, b: usize, c: usize, dp: &mut Vec<Vec<Vec<Option<f64>>>>) -> f64 {
    if let Some(value) = dp[a][b][c] {
        return value;
    }

    if a == 100 || b == 100 || c == 100 {
        return 0.;
    }

    let mut result = 0.;

    result += (rec(a + 1, b, c, dp) + 1.) * a as f64 / (a + b + c) as f64;
    result += (rec(a, b + 1, c, dp) + 1.) * b as f64 / (a + b + c) as f64;
    result += (rec(a, b, c + 1, dp) + 1.) * c as f64 / (a + b + c) as f64;

    dp[a][b][c] = Some(result);

    result
}
