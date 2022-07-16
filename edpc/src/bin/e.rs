use proconio::fastout;
use proconio::input;

const INF: usize = 1 << 60;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        mut wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![INF; 200_001]; 101];

    // 何も選んでない状態で価値0の重さは0
    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..=100_000 {
            // 買わない
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);

            // 買う
            if dp[i][j] + wv[i].0 <= w {
                dp[i + 1][j + wv[i].1] = dp[i + 1][j + wv[i].1].min(dp[i][j] + wv[i].0);
            }
        }
    }

    for i in (1..=100_001).rev() {
        if dp[n][i] != INF {
            println!("{}", i);
            return;
        }
    }
}
