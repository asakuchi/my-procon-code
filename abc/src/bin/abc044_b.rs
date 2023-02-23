use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        w: Chars,
    }

    let mut map = HashMap::new();

    for c in w {
        *map.entry(c).or_insert(0) += 1;
    }

    for (_, count) in map {
        if count % 2 != 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
