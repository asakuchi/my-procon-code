use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        b_c: [(usize, usize); q],
    }

    let mut map = HashMap::new();

    let mut result = 0;

    for x in a {
        *map.entry(x).or_insert(0) += 1;

        result += x;
    }

    for (b, c) in b_c {
        if let Some(old_b_count) = map.insert(b, 0) {
            *map.entry(c).or_insert(0) += old_b_count;

            result -= b * old_b_count;
            result += c * old_b_count;
        }

        println!("{}", result);
    }
}
