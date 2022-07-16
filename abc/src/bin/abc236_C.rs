// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let mut set = std::collections::HashSet::new();

    for express in t {
        set.insert(express);
    }

    for station in s {
        if set.contains(&station) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
