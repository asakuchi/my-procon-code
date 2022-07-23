//!
//! しゃくとり法
//!

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    let mut answer = 0;

    let mut variables = 0;

    let mut r = 0;

    for l in 0..n {
        while r < n {
            let count = map.entry(a[r]).or_insert(0);

            if *count == 0 && variables == k {
                break;
            }

            if *count == 0 {
                variables += 1;
            }

            *count += 1;
            r += 1;
        }

        answer = answer.max(r - l);

        let count = map.entry(a[l]).or_insert(0);

        *count -= 1;

        if *count == 0 {
            variables -= 1;
        }
    }

    println!("{}", answer);
}
