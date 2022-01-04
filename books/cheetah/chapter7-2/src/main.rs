// -*- coding:utf-8-unix -*-

// use proconio::input;
use itertools::izip;

///
/// 第7章 knapsack 動的計画法version
///
fn main() {
    // input! {
    //     n: usize,
    //     weights: [usize; n],
    //     prices: [usize; n],
    // }
    let weights = vec![3, 4, 1, 2, 3];
    let prices = vec![2, 3, 2, 3, 6];

    let weight_limit = 10;
    let mut dp = vec![vec![-1; 11]; 6];

    dp[0][0] = 0;

    for (i, weight, price) in izip!((1..6), weights.iter(), prices.iter()) {
        for j in 0..11 {
            if dp[i - 1][j] != -1 {
                if j + weight <= weight_limit {
                    dp[i][j + weight] = std::cmp::max(dp[i - 1][j] + price, dp[i - 1][j]);
                }
                dp[i][j] = std::cmp::max(dp[i][j], dp[i - 1][j]);
            }
        }
    }

    println!("{}", dp[5].iter().max().unwrap());
}
