use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![0; 1_00_001]; n + 1];

    dp[0][0] = 1;

    for i in 1..=n {
        let mut s = vec![0; k + 2];

        for j in 1..=k + 1 {
            s[j] += s[j - 1] + dp[i - 1][j - 1];
            s[j] %= MOD;
        }

        for j in 0..=k {
            dp[i][j] += MOD + s[j + 1] - s[if j >= a[i - 1] { j - a[i - 1] } else { 0 }];
            dp[i][j] %= MOD;
        }
    }

    println!("{}", dp[n][k]);
}
