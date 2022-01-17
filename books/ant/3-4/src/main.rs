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
    // let dp = [[0; MAX_N]; 1 << MAX_N];

    let result = rec(0, 0, n, &d, &mut dp);

    // println!("{:?}", dp);

    println!("{}", result);
}

fn rec(visited: usize, current: usize, n: usize, d: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
    // println!("{:?}", visited);

    if dp[visited][current] >= 0 {
        return dp[visited][current];
    }

    // println!(
    //     "探索終了？:{} {} {} {}",
    //     visited,
    //     (1 << n),
    //     (1 << n) - 1,
    //     current
    // );

    if visited == (1 << n) - 1 && current == 0 {
        // 探索完了
        dp[visited][current] = 0;
        return 0;
    }

    let mut result = INF;

    for u in 0..n {
        // println!("next :{} {} {}", visited, visited >> u, visited >> u & 1);

        if (visited >> u & 1) != 1 {
            // println!("going next :{} {}", 1 << u, visited | 1 << u);

            result = std::cmp::min(result, rec(visited | 1 << u, u, n, d, dp) + d[current][u]);
        }
    }

    dp[visited][current] = result;

    result
}
