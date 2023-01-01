use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    let mut total = 0;

    for &value in &a {
        *map.entry(value).or_insert(0usize) += 1;
    }

    for (_x, &count) in &map {
        total += count * (count - 1) / 2;
    }

    for i in 0..n {
        let x = a[i];

        let &count = map.get(&x).unwrap();

        if count == 1 {
            println!("{}", total);
        } else {
            println!(
                "{}",
                total - (count * (count - 1) / 2 - (count - 1) * (count - 2) / 2)
            );
        }
    }
}
