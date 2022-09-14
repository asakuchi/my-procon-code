use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a_b: [(usize, usize); n],
    }

    let mut dp = vec![vec![n + 1_000_000; s + 1]; n + 1];

    dp[0][0] = 0;

    for i in 1..=n {
        for j in 0..=s {
            if j >= a_b[i - 1].0 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - a_b[i - 1].0] + 1);
            }

            if j >= a_b[i - 1].1 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - a_b[i - 1].1] + 1);
            }
        }
    }

    if dp[n][s] != n {
        println!("Impossible");
        return;
    }

    let mut result = Vec::new();
    let mut current_price = s;

    for i in (1..=n).rev() {
        if current_price >= a_b[i - 1].0 && dp[i - 1][current_price - a_b[i - 1].0] == i - 1 {
            result.push('A');
            current_price -= a_b[i - 1].0;
        } else {
            result.push('B');
            current_price -= a_b[i - 1].1;
        }
    }

    result.reverse();

    let text = result.iter().join("");

    println!("{}", text);
}
