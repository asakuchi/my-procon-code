use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let atcoder: Vec<_> = "atcoder".chars().collect();

    let mut dp = vec![vec![0; 8]; n + 1];

    for i in 0..n {
        if s[i] == 'a' {
            dp[i][0] = 1;
            break;
        }
    }

    for i in 0..n {
        for j in 0..=7 {
            dp[i + 1][j] += dp[i][j];

            if j <= 6 && s[i] == atcoder[j] {
                dp[i + 1][j + 1] += dp[i][j];
                dp[i + 1][j + 1] %= MOD;
            }

            dp[i + 1][j] %= MOD;
        }
    }

    println!("{}", dp[n][7]);
}
