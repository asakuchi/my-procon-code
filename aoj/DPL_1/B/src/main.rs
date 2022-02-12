use std::io;
use std::io::prelude::*;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();

    let stdin = io::stdin();

    let mut vw = Vec::with_capacity(n);

    for line_result in stdin.lock().lines() {
        let line = line_result.unwrap();

        let mut iter = line.split_whitespace();

        let vi: usize = iter.next().unwrap().parse().unwrap();
        let wi: usize = iter.next().unwrap().parse().unwrap();

        vw.push((vi, wi));
    }

    /* --------------------------------------------------- */

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 0..n {
        for j in 1..=w {
            if vw[i].1 > j {
                dp[i + 1][j] = dp[i][j];
            } else {
                dp[i + 1][j] = std::cmp::max(dp[i][j], dp[i][j - vw[i].1] + vw[i].0);
            }
        }
    }

    println!("{}", dp[n][w]);
}
