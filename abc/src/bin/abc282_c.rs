use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut list = VecDeque::from(s);

    let mut result = Vec::new();

    while let Some(c) = list.pop_front() {
        if c == '"' {
            result.push(c);

            while let Some(kukuri) = list.pop_front() {
                result.push(kukuri);

                if kukuri == '"' {
                    break;
                }
            }
        } else {
            // 括られていない
            if c == ',' {
                result.push('.');
            } else {
                result.push(c);
            }
        }
    }

    let text = result.iter().join("");

    println!("{}", text);
}
