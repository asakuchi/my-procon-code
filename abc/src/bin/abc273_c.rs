use std::collections::HashMap;

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut list = a.clone();

    list.sort();
    list.dedup();
    list.reverse();

    let mut map = HashMap::new();

    for i in 0..list.len() {
        map.insert(list[i], i);
    }

    let mut k_map = HashMap::new();

    for i in 0..n {
        let key = map.get(&a[i]).unwrap();

        // a[i] より大きいものは2種類

        *k_map.entry(key).or_insert(0) += 1;
    }

    for i in 0..n {
        if let Some(value) = k_map.get(&i) {
            println!("{}", value);
        } else {
            println!("0");
        }
    }
}
