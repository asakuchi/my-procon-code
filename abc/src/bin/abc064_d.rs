use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut l_count = 0isize;

    let mut result = VecDeque::new();

    for &c in s.iter() {
        result.push_back(c);

        if c == '(' {
            l_count += 1;
        } else {
            l_count -= 1;
        }

        if l_count < 0 {
            result.push_front('(');
            l_count += 1;
        }
    }

    while l_count > 0 {
        result.push_back(')');
        l_count -= 1;
    }

    for c in result {
        print!("{}", c);
    }

    println!();
}
