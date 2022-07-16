// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 第8章 株式投資シミュレーション
///
fn main() {
    input! {
        initial_investment: usize, // 0 ~ 10000
        monthly_contribution: usize, // 0 ~ 1000
        n: usize, // 2 ~ 50
        m: usize, // stock_prices の要素の要素数 1 ~ 50
        stock_prices: [[usize; m]; n], // 各値は 1~999
    }

    let last_month = &stock_prices[n - 1];

    // 月末から見ていったときの暫定的な最大利益
    let mut stock_temp_max = 0.0;
    // 合計利益
    let mut sum = 0.0;

    for (i, month) in stock_prices.iter().rev().enumerate() {
        if i == 0 {
            // 最終月は考慮することなし
            continue;
        }

        let money = if i == n - 1 {
            initial_investment
        } else {
            monthly_contribution
        };

        for (j, &stock) in month.iter().enumerate() {
            // その月その銘柄での倍率
            let delta = last_month[j] as f64 / stock as f64;

            // その月での倍率より未来でより良い倍率があるなら、
            // そちらを選択する
            // （つまりその月では保留するということ）
            if delta >= stock_temp_max {
                stock_temp_max = delta;
            }
        }

        if stock_temp_max < 1.0 {
            // 利益が出ないなら何もしない
            continue;
        }

        // 利益を求めるので元手を引く（-1.0）
        sum += (stock_temp_max - 1.0) * money as f64;
    }

    println!("{}", sum.round() as i64);
}
