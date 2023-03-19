use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut c = Vec::new();

    for i in 0..n {
        c.push(a[i]);
    }

    for i in 0..m {
        c.push(b[i]);
    }

    c.sort();

    let mut map = HashMap::new();

    for (i, &x) in c.iter().enumerate() {
        map.insert(x, i + 1);
    }

    let mut result_a = Vec::new();

    for i in 0..n {
        result_a.push(map.get(&a[i]).unwrap());
    }

    let text = result_a.iter().format(" ");

    println!("{}", text);

    let mut result_b = Vec::new();

    for i in 0..m {
        result_b.push(map.get(&b[i]).unwrap());
    }

    let text = result_b.iter().format(" ");

    println!("{}", text);
}
