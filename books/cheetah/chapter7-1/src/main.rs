// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        weights: [usize; n],
        prices: [usize; n],
    }

    let weight_limit = 10;
    let mut dp = vec![vec![-1; 11]; 6];

    let price = dfs(&weights, &prices, weight_limit, 0, 0, &mut dp);

    println!("{}", price)
}

fn dfs(
    weights: &Vec<usize>,
    prices: &Vec<usize>,
    limit: usize,
    index: usize,
    current_weight: usize,
    dp: &mut Vec<Vec<isize>>,
) -> usize {
    if index == weights.len() {
        return 0;
    }

    if dp[index][current_weight] != -1 {
        return dp[index][current_weight] as usize;
    }

    // 選択する
    let selected = if current_weight + weights[index] <= limit {
        dfs(
            weights,
            prices,
            limit,
            index + 1,
            current_weight + weights[index],
            dp,
        ) + prices[index]
    } else {
        0
    };

    // 選択しない
    let not_selected = dfs(weights, prices, limit, index + 1, current_weight, dp);

    let max = std::cmp::max(selected, not_selected);

    dp[index][current_weight] = max as isize;

    max
}
