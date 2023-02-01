use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut set = HashSet::new();

    for x in a {
        if set.contains(&x) {
            set.remove(&x);
        } else {
            set.insert(x);
        }
    }

    println!("{}", set.len());
}
