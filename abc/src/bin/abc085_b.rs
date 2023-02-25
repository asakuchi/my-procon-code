use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }

    let mut set = HashSet::new();

    for x in d {
        set.insert(x);
    }

    println!("{}", set.len());
}
