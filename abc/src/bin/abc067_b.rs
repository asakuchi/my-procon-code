use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut l: [usize; n],
    }

    l.sort_by_key(|&x| Reverse(x));

    let result: usize = l[..k].iter().sum();

    println!("{}", result)
}
