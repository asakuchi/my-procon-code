use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    if k == 0 {
        println!("{}", power(m, n, MOD));

        return;
    }

    let mut dp = vec![vec![0; m]; n];

    for i in 0..m {
        dp[0][i] = 1;
    }

    for i in 1..n {
        let mut s = vec![0; 2 * m + 2];

        for l in 0..m {
            s[l + 1] += s[l] + dp[i - 1][l];
        }

        for j in 0..m {
            if j >= k {
                dp[i][j] += s[j - k + 1];
                dp[i][j] %= MOD;
            }

            if j + k < m {
                dp[i][j] += s[m] - s[j + k];
                dp[i][j] %= MOD;
            }
        }
    }

    let mut result = 0;

    for i in 0..m {
        result += dp[n - 1][i];
        result %= MOD
    }

    println!("{}", result);
}

///
/// 繰り返し二乗法
///
fn power(x: usize, n: usize, modulo: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let mut result = power(x * x % modulo, n / 2, modulo);

    if n % 2 != 0 {
        result = result * x % modulo;
    }

    result
}
