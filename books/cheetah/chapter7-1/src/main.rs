// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        weights: [usize; n],
        prices: [usize; n],
    }

    let weight_limit = 10;

    let price = dfs(&weights, &prices, weight_limit, 0, 0);

    println!("{}", price)
}

fn dfs(
    weights: &Vec<usize>,
    prices: &Vec<usize>,
    limit: usize,
    index: usize,
    current_weight: usize,
) -> usize {
    if index == weights.len() {
        return 0;
    }

    // 選択する
    let selected = if current_weight + weights[index] <= limit {
        dfs(
            weights,
            prices,
            limit,
            index + 1,
            current_weight + weights[index],
        ) + prices[index]
    } else {
        0
    };

    // 選択しない
    let not_selected = dfs(weights, prices, limit, index + 1, current_weight);

    std::cmp::max(selected, not_selected)
}
