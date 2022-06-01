use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [f64; n],
    }

    let mut dp = vec![vec![0.; n + 1]; n + 1];

    dp[0][0] = 1.;

    for i in 1..=n {
        for j in 0..n {
            dp[i][j + 1] += dp[i - 1][j] * a[i - 1];
            dp[i][j] += dp[i - 1][j] * (1. - a[i - 1]);
        }
    }

    let mut result = 0.;

    for i in n / 2 + 1..=n {
        result += dp[n][i];
    }

    println!("{}", result);
}
