use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        p: [usize; m + 1],
    }

    let mut map = HashMap::new();

    for i in 0..m {
        map.insert(d[i].clone(), p[i + 1]);
    }

    let mut result = 0_usize;

    for i in 0..n {
        if let Some(&price) = map.get(&c[i]) {
            result += price;
        } else {
            result += p[0];
        }
    }

    println!("{}", result);
}
