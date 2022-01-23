// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 4 * n-1],
    }

    let mut map = std::collections::HashMap::new();

    for value in &a {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map {
        if *value == 3 {
            println!("{}", key);
            break;
        }
    }
}
