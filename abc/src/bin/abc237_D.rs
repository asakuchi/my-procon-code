// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut list = std::collections::VecDeque::new();
    list.push_back(n);

    for (i, &c) in s.iter().enumerate().rev() {
        if c == 'L' {
            list.push_back(i);
        } else {
            list.push_front(i);
        }
    }

    let result: Vec<_> = list.iter().map(|x| x.to_string()).collect();

    println!("{}", result.join(" "));
}
