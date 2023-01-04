use std::collections::HashMap;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = HashMap::new();

    for text in s {
        *map.entry(text).or_insert(0) += 1;
    }

    let &max = map.values().max().unwrap();

    for text in map
        .iter()
        .filter(|&(_k, &v)| v == max)
        .map(|(k, _v)| k)
        .sorted()
    {
        println!("{}", text);
    }
}
