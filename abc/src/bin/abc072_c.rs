use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for x in a {
        if x > 0 {
            *map.entry(x - 1).or_insert(0) += 1;
        }

        *map.entry(x).or_insert(0) += 1;
        *map.entry(x + 1).or_insert(0) += 1;
    }

    let mut result = 0;

    for (_x, count) in map {
        result = result.max(count);
    }

    println!("{}", result);
}
