use proconio::fastout;
use proconio::input;

const MODULO: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut dp = vec![vec![0; 3001]; n + 1];

    dp[0][0] = 1;

    let mut s = vec![0; 3001];

    for i in 0..n {
        s[0] = dp[i][0];

        for j in 1..=3000 {
            s[j] = s[j - 1] + dp[i][j];
        }

        for j in a[i]..=b[i] {
            dp[i + 1][j] += s[j];
            dp[i + 1][j] %= MODULO;
        }
    }

    let mut result = 0;

    for i in 0..=3000 {
        result += dp[n][i];
        result %= MODULO;
    }

    println!("{}", result);
}
