use std::{cmp::Reverse, collections::HashMap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for x in a {
        *map.entry(x).or_insert(0) += 1;
    }

    let list: Vec<_> = map.into_iter().sorted_by_key(|x| Reverse(x.1)).collect();

    if list.len() <= k {
        println!("0");
        return;
    }

    let mut result = 0usize;

    for &(_x, count) in &list[k..] {
        result += count;
    }

    println!("{}", result);
}
