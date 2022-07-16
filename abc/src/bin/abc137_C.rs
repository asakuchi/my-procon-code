// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }

    let mut hash = std::collections::HashMap::new();

    for mut text in s {
        text.sort();

        *hash.entry(text).or_insert(0) += 1;
    }

    let mut result: u64 = 0;

    for &value in hash.values() {
        if value >= 2 {
            // 組み合わせ
            result += value * (value - 1) / 2;
        }
    }

    println!("{}", result);
}
