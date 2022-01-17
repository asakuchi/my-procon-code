// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 002 - Encyclopedia of Parentheses（★3）
///
fn main() {
    input! {
        n: u32,
    }

    let letters = vec!["(", ")"];

    let mut list = Vec::new();

    'outer: for i in 0..2u32.pow(n) - 1 {
        let mut text = String::from("");

        let mut left_count = 0;
        let mut right_count = 0;

        for j in 0..n {
            let bit = i >> j & 1;

            if bit == 0 {
                left_count += 1;
            } else {
                right_count += 1;
            }

            if right_count > left_count {
                continue 'outer;
            }

            text.push_str(letters[(i >> j & 1) as usize])
        }

        if left_count != right_count {
            continue 'outer;
        }

        list.push(text);
    }

    list.sort();

    for text in list {
        println!("{}", text);
    }
}
