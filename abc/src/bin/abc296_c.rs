use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
        a: [isize; n],
    }

    let mut set = HashSet::new();

    for i in 0..n {
        set.insert(a[i]);
    }

    for i in 0..n {
        let a_j = a[i] - x;

        if set.contains(&a_j) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
