use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut called = vec![false; n];

    for i in 0..n {
        if !called[i] {
            called[a[i]] = true;
        }
    }

    let mut result = Vec::new();

    for i in 0..n {
        if !called[i] {
            result.push(i);
        }
    }

    println!("{}", result.len());

    let text = result.iter().map(|i| i + 1).format(" ");

    println!("{}", text);
}
