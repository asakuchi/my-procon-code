//!
//! LIS:Longest Increasing Subsequence
//! 最長増加部分列
//!

use std::io;
// use superslice::*;

const INF: usize = 1_000_000_000_000;

fn main() {
    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    // ------------------------------------

    let mut a: Vec<usize> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let data: usize = buf.parse().unwrap();

        a.push(data);
    }

    // ------------------------------------

    let mut dp = vec![INF; n + 1];

    for i in 0..n {
        // let index = dp.lower_bound(&a[i]);
        // dp[index] = a[i];

        let mut ok = n as isize;
        let mut ng = -1;

        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;

            let solve = || a[i] <= dp[mid as usize];

            if solve() {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        dp[ok as usize] = a[i];
    }

    // println!("{}", dp.lower_bound(&INF));

    let mut ok = n as isize;
    let mut ng = -1;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        let solve = || INF <= dp[mid as usize];

        if solve() {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
