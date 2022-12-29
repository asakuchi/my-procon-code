use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for &x in &a {
        *map.entry(x).or_insert(0) += 1;
    }

    let mut result = map.len();

    let mut extra_count = 0;

    for (_, &count) in &map {
        if count > 1 {
            extra_count += count - 1;
        }
    }

    if extra_count % 2 == 1 {
        result -= 1;
    }

    println!("{}", result);
}
