use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut set = HashSet::new();

    let mut current = (0, 0);

    set.insert(current);

    for c in s {
        match c {
            'R' => current.0 += 1isize,
            'L' => current.0 -= 1isize,
            'U' => current.1 += 1isize,
            _ => current.1 -= 1isize,
        }

        if set.contains(&current) {
            println!("Yes");
            return;
        }

        set.insert(current);
    }

    println!("No");
}
