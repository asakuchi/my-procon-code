use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut set = BTreeSet::new();

    let mut result = 0;

    for value in p {
        set.insert(value);

        if let Some(_next) = set.range(..value).next() {
            // NG
        } else {
            result += 1;
        }
    }

    println!("{}", result);
}
