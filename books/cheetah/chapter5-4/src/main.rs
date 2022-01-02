// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// TODO: 振り返り
///
fn main() {
    input! {
        s: String,
    }

    let mut result = std::usize::MAX;

    'outer: for i in s.len()..std::usize::MAX {
        for j in 0..s.len() {
            if (i - j - 1) < s.len()
                && s.chars().nth(j).unwrap() != s.chars().nth(i - j - 1).unwrap()
            {
                continue 'outer;
            }
        }

        result = i;

        break;
    }

    println!("{:?}", result);
}
