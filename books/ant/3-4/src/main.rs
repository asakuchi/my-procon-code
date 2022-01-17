// -*- coding:utf-8-unix -*-

use proconio::input;

const MAX_N: usize = 15;
const INF: i32 = 1_000 * 20; // 十分大きい数

///
/// ビットDP
/// 巡回セールスマン問題
///
fn main() {
    input! {
        n: usize,
        mut d: [[i32; n]; n],
    }

    // 入力値を修正
    for i in 0..n {
        for j in 0..n {
            if d[i][j] == -1 {
                d[i][j] = INF;
            }
        }
    }

    let mut dp = vec![vec![-1; MAX_N]; 1 << MAX_N];

    let result = rec(0, 0, n, &d, &mut dp);

    println!("{}", result);
}

fn rec(visited: usize, current: usize, n: usize, d: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
    if dp[visited][current] >= 0 {
        return dp[visited][current];
    }

    if visited == (1 << n) - 1 && current == 0 {
        // 探索完了
        dp[visited][current] = 0;
        return 0;
    }

    let mut result = INF;

    for u in 0..n {
        if visited & 1 << u == 0 {
            let u_score = rec(visited | 1 << u, u, n, d, dp) + d[current][u];

            result = std::cmp::min(result, u_score);
        }
    }

    dp[visited][current] = result;

    result
}
