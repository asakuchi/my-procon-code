use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        mut s: Chars,
    }

    let atcoder: Vec<_> = "atcoder".chars().enumerate().collect();

    let mut order = HashMap::new();

    for &(i, c) in &atcoder {
        order.entry(c).or_insert(i);
    }

    let mut result = 0;

    for i in 0..7 {
        for j in 1..7 - i {
            if order.get(&s[j - 1]).unwrap() > order.get(&s[j]).unwrap() {
                s.swap(j - 1, j);
                result += 1;
            }
        }
    }

    println!("{}", result);
}
