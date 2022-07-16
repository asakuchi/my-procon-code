use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        mut wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; 100_001]; 101];

    for i in 0..n {
        for j in 1..=w {
            // 買わない
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);

            // 買う
            if j >= wv[i].0 {
                dp[i + 1][j - wv[i].0] = dp[i + 1][j - wv[i].0].max(dp[i][j] + wv[i].1);
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
