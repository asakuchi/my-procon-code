use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut list = VecDeque::from(a);

    for _ in 0..k {
        list.pop_front();
        list.push_back(0);
    }

    let result = list.iter().format(" ");

    println!("{}", result);
}
