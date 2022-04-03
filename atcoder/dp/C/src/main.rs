use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut plan: [[usize;3]; n],
    }

    let mut dp = vec![vec![0; 3]; n + 1];

    for i in 0..3 {
        dp[0][i] = plan[0][i];
    }

    for i in 1..n {
        dp[i][0] = dp[i - 1][1].max(dp[i - 1][2]) + plan[i][0];
        dp[i][1] = dp[i - 1][0].max(dp[i - 1][2]) + plan[i][1];
        dp[i][2] = dp[i - 1][0].max(dp[i - 1][1]) + plan[i][2];
    }

    println!("{}", dp[n - 1][0].max(dp[n - 1][1].max(dp[n - 1][2])));
}
