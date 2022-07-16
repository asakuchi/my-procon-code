use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

const MODULO: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut dp = vec![vec![0; w]; h];

    dp[0][0] = 1;

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '.' {
                if i > 0 {
                    dp[i][j] += dp[i - 1][j];
                    dp[i][j] %= MODULO;
                }

                if j > 0 {
                    dp[i][j] += dp[i][j - 1];
                    dp[i][j] %= MODULO;
                }
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
