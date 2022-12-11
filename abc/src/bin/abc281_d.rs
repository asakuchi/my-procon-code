use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![vec![None; d]; k + 1]; n + 1];

    dp[0][0][0] = Some(0);

    for i in 0..n {
        for j in 0..k + 1 {
            for l in 0..d {
                if let Some(prev) = dp[i][j][l] {
                    // 選ばない場合
                    if let Some(next) = dp[i + 1][j][l] {
                        dp[i + 1][j][l] = Some(next.max(prev));
                    } else {
                        dp[i + 1][j][l] = Some(prev);
                    }

                    // 選ぶ場合
                    if j != k {
                        if let Some(next) = dp[i + 1][j + 1][(l + a[i]) % d] {
                            dp[i + 1][j + 1][(l + a[i]) % d] = Some(next.max(prev + a[i]));
                        } else {
                            dp[i + 1][j + 1][(l + a[i]) % d] = Some(prev + a[i]);
                        }
                    }
                }
            }
        }
    }

    if let Some(value) = dp[n][k][0] {
        println!("{}", value);
    } else {
        println!("-1");
    }
}
