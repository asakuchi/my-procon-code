// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize, // first と second の要素数
        first: [String;n],
        second: [String;n],
    }

    let concatenated = [&first[..], &second[..]].concat();

    let mut hash = std::collections::HashMap::new();

    for interest in concatenated {
        let count = hash.entry(interest).or_insert(0);
        *count += 1;
    }

    let max = hash.values().max().unwrap();

    println!("{}", max);
}
