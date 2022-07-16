//!
//! LCS:Longest Common Subsequence
//! 最長共通部分列
//!
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    }

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else if dp[i - 1][j] >= dp[i][j - 1] {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }

    let mut result = Vec::new();

    let mut len = dp[s.len()][t.len()];
    let mut i = s.len();
    let mut j = t.len();

    while len > 0 {
        if s[i - 1] == t[j - 1] {
            result.push(s[i - 1]);
            i -= 1;
            j -= 1;
            len -= 1;
        } else if dp[i][j] == dp[i - 1][j] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    println!(
        "{}",
        result
            .iter()
            .rev()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
    );
}
