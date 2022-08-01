use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 0;

    for i in 1..=n {
        let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n + 1];

        // aの先頭0項から0項選ぶとき、総和をiで割った余りが0になる個数は1
        dp[0][0][0] = 1;

        for j in 0..n {
            for k in 0..=i {
                for l in 0..i {
                    dp[j + 1][k][l] += dp[j][k][l];
                    dp[j + 1][k][l] %= MOD;

                    if k != i {
                        dp[j + 1][k + 1][(l + a[j]) % i] += dp[j][k][l];
                        dp[j + 1][k + 1][(l + a[j]) % i] %= MOD;
                    }
                }
            }
        }

        result += dp[n][i][0];
        result %= MOD;
    }

    println!("{}", result);
}
