//!
//! しゃくとり法
//!

use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    }

    let mut result = 0;
    let mut list = VecDeque::new();

    let mut dot = 0;
    let mut cross = 0;

    for value in s {
        list.push_back(value);

        if value == '.' {
            dot += 1;
        } else {
            cross += 1;
        }

        while dot > k {
            if let Some(left) = list.pop_front() {
                if left == '.' {
                    dot -= 1;
                } else {
                    cross -= 1;
                }
            } else {
                break;
            }
        }

        result = result.max(cross + dot);
    }

    println!("{}", result);
}
