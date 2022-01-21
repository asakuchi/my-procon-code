// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = std::collections::HashMap::new();

    for name in s {
        let count = map.entry(name).or_insert(0);
        *count += 1;
    }

    let max_name = map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();

    println!("{}", max_name);
}
