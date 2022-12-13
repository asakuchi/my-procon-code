use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![None; 21]; n];
    // dp[0][0] = Some(1usize);
    dp[1][a[0]] = Some(1usize);

    for i in 1..n - 1 {
        for j in 0..=20 {
            // +
            if j + a[i] <= 20 {
                if let Some(current) = dp[i][j] {
                    if let Some(next) = dp[i + 1][j + a[i]] {
                        dp[i + 1][j + a[i]] = Some(next + current);
                    } else {
                        dp[i + 1][j + a[i]] = Some(current);
                    }
                }
            }

            // -
            if j >= a[i] {
                if let Some(current) = dp[i][j] {
                    if let Some(next) = dp[i + 1][j - a[i]] {
                        dp[i + 1][j - a[i]] = Some(next + current);
                    } else {
                        dp[i + 1][j - a[i]] = Some(current);
                    }
                }
            }
        }
    }

    // println!("{:?}", dp[n - 1]);

    if let Some(result) = dp[n - 1][a[n - 1]] {
        println!("{}", result);
    } else {
        println!("0");
    }
}
