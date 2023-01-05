use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut set = HashSet::with_capacity(n);

    for value in a {
        set.insert(value);
    }

    if set.len() == n {
        println!("YES");
    } else {
        println!("NO");
    }
}
