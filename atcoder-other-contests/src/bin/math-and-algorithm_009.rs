use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![0; 601_000]; n + 1];

    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..600_000 {
            if j + a[i] > s {
                continue;
            }
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j + a[i]] += dp[i][j];
        }
    }

    if dp[n][s] > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
