use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        s: Chars
    }

    let chokudai = vec!['c', 'h', 'o', 'k', 'u', 'd', 'a', 'i'];

    let n = s.len();

    let mut dp = vec![vec![0; chokudai.len() + 1]; n + 1];

    for i in 0..=n {
        // 1文字も使わない
        dp[i][0] = 1;
    }

    for i in 1..=n {
        for j in 1..=chokudai.len() {
            dp[i][j] += dp[i - 1][j];
            dp[i][j] %= MOD;

            if s[i - 1] == chokudai[j - 1] {
                dp[i][j] += dp[i - 1][j - 1];
                dp[i][j] %= MOD;
            }
        }
    }

    // println!("{:?}", dp);

    println!("{}", dp[n][chokudai.len()]);
}
