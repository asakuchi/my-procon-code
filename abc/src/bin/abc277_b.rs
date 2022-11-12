use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let first = vec!['H', 'D', 'C', 'S'];
    let second = vec![
        'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
    ];

    let mut set = HashSet::new();

    for text in s {
        if !first.contains(&text[0]) {
            println!("No");
            return;
        }

        if !second.contains(&text[1]) {
            println!("No");
            return;
        }

        set.insert(text.clone());
    }

    if set.len() != n {
        println!("No");
        return;
    }

    println!("Yes");
}
