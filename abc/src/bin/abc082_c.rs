use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for x in a {
        *map.entry(x).or_insert(0) += 1;
    }

    let mut result = 0;

    for (x, count) in map {
        if x == count {
            // OK
            continue;
        }

        if x > count {
            result += count;
            continue;
        }

        result += count - x;
    }

    println!("{}", result);
}
