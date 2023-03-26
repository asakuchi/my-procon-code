use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut acc = vec![vec![0; 10]; s.len() + 1];

    for i in 0..s.len() {
        for j in 0..=9 {
            acc[i + 1][j] += acc[i][j];

            if j.to_string() == s[i].to_string() {
                acc[i + 1][j] += 1;
            }

            acc[i + 1][j] %= 2;
        }
    }

    let mut map = HashMap::new();

    for i in 0..=s.len() {
        *map.entry(acc[i].clone()).or_insert(0) += 1;
    }

    let mut result = 0;

    for (_, &count) in map.iter() {
        result += combination(count, 2);
    }

    println!("{}", result);
}

fn combination(n: usize, k: usize) -> usize {
    let mut result = 1;

    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }

    result
}
