use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut set = HashSet::new();

    let mut result = 0;

    for i in 0..n {
        let rev: Vec<_> = s[i].clone().into_iter().rev().collect();

        if !set.contains(&s[i]) && !set.contains(&rev) {
            result += 1;
        }

        set.insert(s[i].clone());
        set.insert(rev.clone());
    }

    println!("{}", result);
}
