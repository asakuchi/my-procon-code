use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
    }

    // 長さiのAの部分列jの最大値
    let mut dp = vec![vec![0; m + 1]; n + 1];

    dp[0][1] = -1000000000000000000;

    for i in 1..=n {
        for j in 1..=m {
            if j > i {
                dp[i][j] = -1000000000000000000;
                continue;
            }

            dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - 1] + a[i - 1] * j as isize);
        }
    }

    println!("{:?}", dp);

    println!("{}", dp[n][m]);
}
