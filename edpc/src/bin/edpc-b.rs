use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [isize; n],
    }

    let mut dp = vec![1 << 60; 1000000];

    dp[0] = 0;

    for i in 0..n {
        for j in 1..=k {
            if i + j < n {
                dp[i + j] = dp[i + j].min(dp[i] + (h[i] - h[i + j]).abs());
            }
        }
    }

    println!("{}", dp[n - 1]);
}
