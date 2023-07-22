use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut set = HashSet::new();

    for i in 0..n {
        set.insert(s[i]);

        if set.len() == 3 {
            println!("{}", i + 1);
            return;
        }
    }
}
