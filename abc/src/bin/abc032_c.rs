use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [usize; n],
    }

    if s.iter().any(|&x| x == 0) {
        println!("{}", n);
        return;
    }

    let mut result = 0;
    let mut list = VecDeque::new();
    let mut p = 1;

    for value in s {
        list.push_back(value);
        p *= value;

        while p > k {
            if let Some(left) = list.pop_front() {
                p /= left;
            } else {
                break;
            }
        }

        result = result.max(list.len());
    }

    println!("{}", result);
}
