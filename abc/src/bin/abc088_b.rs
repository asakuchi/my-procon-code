use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }

    a.sort_by_key(|&x| Reverse(x));

    let mut result = 0;

    for i in 0..n {
        if i % 2 == 0 {
            result += a[i];
        } else {
            result -= a[i];
        }
    }

    println!("{}", result);
}
