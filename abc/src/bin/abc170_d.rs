use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut counter = HashMap::new();

    for i in 0..n {
        *counter.entry(a[i]).or_insert(0) += 1;
    }

    let mut ng = HashSet::new();

    for (&x, &count) in counter.iter() {
        if count > 1 {
            ng.insert(x);
        }

        for mul in (x * 2..=1_000_000).step_by(x) {
            ng.insert(mul);
        }
    }

    let mut result = 0;

    for i in 0..n {
        let x = a[i];

        if !ng.contains(&x) {
            result += 1;
        }
    }

    println!("{}", result);
}
