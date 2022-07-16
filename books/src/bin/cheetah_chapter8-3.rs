// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 第8章 バッチシステム
///
fn main() {
    input! {
        n: usize, // ユーザ数
        duration: [usize; n],
        user: [String; n],
    }

    // ユーザごとの合計時間を求める
    let mut hash = std::collections::HashMap::new();

    for (d, u) in duration.iter().zip(user.iter()) {
        let value = hash.entry(u).or_insert(0);
        *value += d;
    }

    // 結果
    let mut result = Vec::new();

    // 結果にジョブ番号を追加したか
    let mut done = vec![false; n];

    while result.len() < n {
        let mut next_user = String::from("");

        // 合計時間が多い順にユーザを取得したいが、
        // 合計時間が同じ場合はジョブ番号順に出力する必要があるので、
        // user の並びを考慮する
        for (i, user_name) in user.iter().enumerate() {
            if !done[i] && (next_user == "" || hash[user_name] < hash[&next_user]) {
                next_user = user_name.clone();
            }
        }

        for i in 0..n {
            if user[i] == next_user {
                done[i] = true;
                result.push(i);
            }
        }
    }

    println!("{:?}", result);
}
