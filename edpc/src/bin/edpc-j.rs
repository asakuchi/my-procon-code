use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut counter = vec![0; 4];

    for i in 0..n {
        counter[a[i]] += 1;
    }

    // dp[i][j][k]
    // 寿司が1個の皿がi個、寿司が2個の皿がj個、寿司が3個の皿がk個、の期待値
    let mut dp = vec![vec![vec![0.; n + 3]; n + 3]; n + 3];

    for k in 0..=n {
        for j in 0..=n {
            for i in 0..=n {
                if i + j + k == 0 {
                    continue;
                }

                dp[i][j][k] = n as f64 / (i + j + k) as f64;

                if i >= 1 {
                    dp[i][j][k] += dp[i - 1][j][k] * i as f64 / (i + j + k) as f64;
                }
                if j >= 1 {
                    dp[i][j][k] += dp[i + 1][j - 1][k] * j as f64 / (i + j + k) as f64
                }
                if k >= 1 {
                    dp[i][j][k] += dp[i][j + 1][k - 1] * k as f64 / (i + j + k) as f64;
                }
            }
        }
    }

    println!("{}", dp[counter[1]][counter[2]][counter[3]]);
}
