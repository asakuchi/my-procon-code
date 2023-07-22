use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a_b: [(usize, usize); n],
    }

    let mut ng = HashSet::new();

    for (a, b) in a_b {
        ng.insert((a, b));
    }

    let mut dp = vec![vec![0; w + 1]; h + 1];

    let mut result = 0_usize;

    for i in 1..=h {
        for j in 1..=w {
            if ng.contains(&(i, j)) {
                dp[i][j] = 0;
            } else {
                dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]) + 1;
            }

            result += dp[i][j];
        }
    }

    println!("{}", result);
}
