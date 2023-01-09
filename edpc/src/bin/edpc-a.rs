use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut h: [isize; n],
    }

    let mut dp = vec![1 << 60; 1000000];

    dp[0] = 0;

    for i in 0..n {
        if i + 1 < n {
            dp[i + 1] = dp[i + 1].min(dp[i] + (h[i] - h[i + 1]).abs());
        }
        if i + 2 < n {
            dp[i + 2] = dp[i + 2].min(dp[i] + (h[i] - h[i + 2]).abs());
        }
    }

    println!("{}", dp[n - 1]);
}
