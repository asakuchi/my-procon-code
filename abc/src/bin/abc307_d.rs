use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut list = VecDeque::new();

    let mut current = Vec::new();

    for c in s {
        if c == '(' {
            list.push_back(current.clone());
            current = Vec::new();
            current.push('(');
        } else if c == ')' {
            if let Some(tail) = list.pop_back() {
                current = tail;
            } else {
                current.push(')');
            }
        } else {
            current.push(c);
        }
    }

    let mut result = Vec::new();

    list.push_back(current);

    while let Some(value) = list.pop_front() {
        for c in value {
            result.push(c);
        }
    }

    let text = result.iter().join("");

    println!("{}", text);
}
