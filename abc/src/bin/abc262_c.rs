use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 0;

    let mut same_index = HashSet::new();

    for i in 0..n {
        if a[i] == i + 1 {
            same_index.insert(i);
        }
    }

    for i in 0..n {
        if a[i] == i + 1 {
            result += same_index.len() - 1;
        } else if a[a[i] - 1] == i + 1 {
            result += 1;
        }
    }

    println!("{}", result / 2);
}
