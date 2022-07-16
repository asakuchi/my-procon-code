// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 第7章 仲の悪い隣人たち
///
fn main() {
    input! {
        n: usize,
        donations: [usize; n],
    }

    let mut memo = vec![vec![vec![-1; 2]; 2]; n];

    let max = dfs(&donations, 0, false, false, &mut memo);

    println!("{}", max);
}

fn dfs(
    donations: &Vec<usize>,
    index: usize,
    previous_donated: bool,
    first_donated: bool,
    memo: &mut Vec<Vec<Vec<i32>>>,
) -> usize {
    if index == donations.len() {
        return 0;
    }

    if memo[index][if previous_donated { 1 } else { 0 }][if first_donated { 1 } else { 0 }] != -1 {
        return memo[index][if previous_donated { 1 } else { 0 }][if first_donated { 1 } else { 0 }]
            as usize;
    }

    if previous_donated {
        // 隣人が寄付したらこの住人は寄付しない
        let value = dfs(donations, index + 1, false, first_donated, memo);

        memo[index][if previous_donated { 1 } else { 0 }][if first_donated { 1 } else { 0 }] =
            value as i32;

        return value;
    }

    if index == donations.len() - 1 && first_donated {
        // 最初の住人が寄付したら最後の住人は寄付しない
        // （最初の住人と最後の住人は隣人）
        let value = dfs(donations, index + 1, false, first_donated, memo);

        memo[index][if previous_donated { 1 } else { 0 }][if first_donated { 1 } else { 0 }] =
            value as i32;

        return value;
    }

    let selected = if index == 0 {
        // 最初の住人が寄付したかどうかを記録
        // 最後の住人の寄付でチェック
        dfs(donations, index + 1, true, true, memo) + donations[index]
    } else {
        dfs(donations, index + 1, true, first_donated, memo) + donations[index]
    };

    let unselected = dfs(donations, index + 1, false, first_donated, memo);

    let value = std::cmp::max(selected, unselected);

    memo[index][if previous_donated { 1 } else { 0 }][if first_donated { 1 } else { 0 }] =
        value as i32;

    value
}
