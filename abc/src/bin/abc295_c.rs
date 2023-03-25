use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for &color in &a {
        *map.entry(color).or_insert(0) += 1;
    }

    let mut result = 0_usize;

    for (_, &count) in map.iter() {
        result += count / 2;
    }

    println!("{}", result);
}
