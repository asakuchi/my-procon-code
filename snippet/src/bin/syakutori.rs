//!
//! しゃくとり法
//! https://atcoder.jp/contests/typical90/tasks/typical90_ah
//!

use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut result = 0;
    let mut list = VecDeque::new();

    // --- 問題固有の変数 ---
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut variables = 0;
    // --- 問題固有の変数 ---

    for (right, &right_value) in a.iter().enumerate() {
        list.push_back((right, right_value));

        // -- 問題固有の処理
        let count = map.entry(right_value).or_insert(0);
        if *count == 0 {
            variables += 1;
        }
        *count += 1;
        // -- 問題固有の処理

        while variables > k {
            if let Some((_left, left_value)) = list.pop_front() {
                // -- 問題固有の処理
                let count = map.entry(left_value).or_insert(0);
                *count -= 1;
                if *count == 0 {
                    variables -= 1;
                }
                // -- 問題固有の処理
            } else {
                break;
            }
        }

        // 条件を満たす部分列
        // println!("{:?}", list);

        // 問題固有の判定
        result = result.max(list.len());
    }

    println!("{}", result);
}
