use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [usize; n],
    }

    let list: Vec<_> = v.iter().sorted().map(|&x| x as f64).collect();

    let mut list = VecDeque::from(list);

    let mut result = list.pop_front().unwrap();

    while let Some(item) = list.pop_front() {
        result = (result + item) / 2.;
    }

    println!("{:.8}", result);
}
