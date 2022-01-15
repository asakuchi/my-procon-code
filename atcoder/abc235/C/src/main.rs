// -*- coding:utf-8-unix -*-

use proconio::input;
// use proconio::derive_readable;
// use proconio::marker::Chars;
// use itertools::izip;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xk: [(usize, usize); q],
    }

    let mut hash = std::collections::HashMap::new();
    let mut memory = std::collections::HashMap::new();

    for i in 0..n {
        let number = a[i];

        let count = hash.entry(number).or_insert(0);
        *count += 1;

        memory.insert((number, count.clone()), i);
    }

    for (x, k) in xk {
        match memory.get(&(x, k)) {
            Some(number) => println!("{}", number + 1),
            None => println!("-1"),
        }
    }
}
