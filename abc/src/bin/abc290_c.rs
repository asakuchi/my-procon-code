use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for &x in &a {
        *map.entry(x).or_insert(0) += 1;
    }

    for x in 0..k {
        if let Some(_value) = map.get(&x) {
            // do nothin.
        } else {
            println!("{}", x);
            return;
        }
    }

    println!("{}", k);
}
