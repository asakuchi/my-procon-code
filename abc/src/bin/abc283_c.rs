use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut list = VecDeque::from(s);

    let mut result = 0usize;

    if let Some(c) = list.pop_front() {
        if c == '0' {
            println!("0");
            return;
        }

        list.push_front(c);
    }

    while let Some(c) = list.pop_front() {
        if c != '0' {
            result += 1;
        } else {
            if let Some(c_next) = list.pop_front() {
                if c_next == '0' {
                    // 00
                    result += 1;
                } else {
                    // 0
                    result += 1;

                    list.push_front(c_next);
                }
            } else {
                // 0
                result += 1;
            }
        }
    }

    println!("{}", result);
}
