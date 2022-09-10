use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a: [usize; 5],
    }

    let mut set = HashSet::new();

    for value in a {
        set.insert(value);
    }

    println!("{}", set.len());
}
