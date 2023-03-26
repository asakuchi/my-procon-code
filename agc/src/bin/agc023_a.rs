use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut acc = vec![0; n + 1];

    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }

    let mut map = HashMap::new();

    for i in 0..=n {
        *map.entry(acc[i]).or_insert(0) += 1;
    }

    let mut result = 0usize;

    for (_, &count) in map.iter() {
        result += count * (count - 1) / 2;
    }

    println!("{}", result);
}
