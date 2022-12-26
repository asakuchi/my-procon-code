use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut minus_map = HashMap::new();

    for i in 0..n {
        *minus_map.entry(-a[i] + i as isize).or_insert(0isize) += 1;
    }

    let mut result = 0;

    for i in 0..n {
        if let Some(&value) = minus_map.get(&(a[i] + i as isize)) {
            result += value;
        }
    }

    println!("{}", result);
}
