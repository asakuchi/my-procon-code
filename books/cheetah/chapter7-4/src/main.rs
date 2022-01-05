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

    let max = dfs(&donations, 0, false, false);

    println!("{}", max);
}

fn dfs(donations: &Vec<usize>, index: usize, previous_donated: bool, first_donated: bool) -> usize {
    if index == donations.len() {
        return 0;
    }

    if previous_donated {
        // 隣人が寄付したらこの住人は寄付しない
        return dfs(donations, index + 1, false, first_donated);
    }

    if index == donations.len() - 1 && first_donated {
        // 最初の住人が寄付したら最後の住人は寄付しない
        // （最初の住人と最後の住人は隣人）
        return dfs(donations, index + 1, false, first_donated);
    }

    let selected = if index == 0 {
        // 最初の住人が寄付したかどうかを記録
        // 最後の住人の寄付でチェック
        dfs(donations, index + 1, true, true) + donations[index]
    } else {
        dfs(donations, index + 1, true, first_donated) + donations[index]
    };

    let unselected = dfs(donations, index + 1, false, first_donated);

    std::cmp::max(selected, unselected)
}
