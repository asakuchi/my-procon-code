// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i128,
        a: [i128; n],
    }

    let mut s = vec![0; n + 1];

    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let mut map = std::collections::HashMap::new();

    let mut answer = 0;

    for i in 0..n + 1 {
        if let Some(value) = map.get(&(s[i])) {
            answer += value;
        }
        let value = map.entry(s[i] + k).or_insert(0);
        *value += 1;
    }

    println!("{}", answer);
}
