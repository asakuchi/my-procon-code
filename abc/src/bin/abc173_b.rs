use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = HashMap::new();

    for text in s {
        *map.entry(text).or_insert(0) += 1;
    }

    let cases = vec!["AC", "WA", "TLE", "RE"];

    for case in cases {
        println!("{} x {}", case, map.entry(case.to_string()).or_insert(0));
    }
}
