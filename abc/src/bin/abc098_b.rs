use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut pre = HashMap::new();
    let mut post = HashMap::new();

    for i in 0..n {
        *post.entry(s[i]).or_insert(0) += 1;
    }

    let mut result = 0;

    for i in 0..n {
        let counter = post.entry(s[i]).or_insert(0);

        *counter -= 1;

        if *counter == 0 {
            post.remove(&s[i]);
        }

        *pre.entry(s[i]).or_insert(0) += 1;

        let mut kind = 0;

        for c in 'a' as u8..='z' as u8 {
            if pre.contains_key(&(c as char)) && post.contains_key(&(c as char)) {
                kind += 1;
            }
        }

        result = result.max(kind);
    }

    println!("{}", result);
}
