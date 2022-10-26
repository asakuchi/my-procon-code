use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [char; n],
    }

    let mut set = HashSet::with_capacity(n);

    for c in s {
        set.insert(c);
    }

    if set.len() == 3 {
        println!("Three");
    } else {
        println!("Four");
    }
}
