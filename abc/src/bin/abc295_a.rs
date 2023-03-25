use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    }

    let mut set = HashSet::new();

    set.insert("and");
    set.insert("not");
    set.insert("that");
    set.insert("the");
    set.insert("you");

    for text in w {
        if set.contains(&text.as_str()) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
